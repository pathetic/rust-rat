use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use base64::{engine::general_purpose, Engine as _};
use tauri::{AppHandle, Manager};

use common::buffers::{read_buffer, write_buffer};
use common::commands::{Command, File};

#[derive(Debug)]
pub struct Client {
    tauri_handle: Arc<Mutex<AppHandle>>,
    write_stream: TcpStream,
    read_stream: Arc<Mutex<TcpStream>>,

    username: String,
    hostname: String,
    os: String,
    ram: String,
    cpu: String,
    gpus: Vec<String>,
    storage: Vec<String>,
    displays: i32,
    ip: String,

    is_elevated: bool,

    disconnected: Arc<Mutex<bool>>,

    pub is_handled: bool,

    pub shell_started: bool,

    files: Arc<Mutex<Vec<File>>>,
    current_path: Arc<Mutex<String>>,
}

impl Client {
    pub fn new(
        tauri_handle: Arc<Mutex<AppHandle>>,
        write_stream: TcpStream,
        username: String,
        hostname: String,
        os: String,
        ram: String,
        cpu: String,
        gpus: Vec<String>,
        storage: Vec<String>,
        displays: i32,
        ip: String,
        is_elevated: bool,
        files: Arc<Mutex<Vec<File>>>,
        current_path: Arc<Mutex<String>>,
    ) -> Self {
        Client {
            tauri_handle,
            write_stream: write_stream.try_clone().unwrap(),
            read_stream: Arc::new(Mutex::new(write_stream.try_clone().unwrap())),
            username,
            hostname,
            os,
            ram,
            cpu,
            gpus,
            storage,
            displays,
            ip,
            is_elevated,
            disconnected: Arc::new(Mutex::new(false)),
            is_handled: false,
            shell_started: false,
            files,
            current_path,
        }
    }

    pub fn write_buffer(&mut self, command: Command) {
        write_buffer(&mut self.write_stream, command)
    }

    pub fn handle_client(&mut self) {
        let username_clone = self.username.clone();
        let stream_clone = Arc::clone(&self.read_stream);
        let files = Arc::clone(&self.files);
        let current_path = Arc::clone(&self.current_path);
        let disconnected = Arc::clone(&self.disconnected);
        let tauri_handle = Arc::clone(&self.tauri_handle);

        std::thread::spawn(move || {
            loop {
                let mut locked_stream = stream_clone.lock().unwrap();
                let received_data = read_buffer(&mut locked_stream);

                match received_data {
                    Ok(received) => {
                        match received {
                            Command::ProcessList(process_list) => {
                                let _ = tauri_handle.lock().unwrap().emit_all("process_list", process_list);
                            }
                            Command::ShellOutput(shell_output) => {
                                let _ = tauri_handle.lock().unwrap().emit_all("client_shellout", shell_output);
                            }
                            Command::ScreenshotResult(screenshot) => {
                                let base64_img = general_purpose::STANDARD.encode(screenshot);
                                let _ = tauri_handle.lock().unwrap().emit_all("client_screenshot", base64_img);
                            }
                            Command::DisksResult(disks) => {
                                for disk in disks {
                                    files.lock().unwrap().push(File{file_type: "dir".to_string(), name: format!("{}:\\", disk)});
                                }
                            }
                            Command::CurrentFolder(folder) => {
                                *current_path.lock().unwrap() = folder;
                            }
                            Command::FileList(files_list) => {
                                let mut files = files.lock().unwrap();
                                files.clear();
                                for file in files_list {
                                    files.push(file);
                                }
                            }
                            Command::DonwloadFileResult(data) => {
                                let _ = std::fs::write(data.name, data.data);
                            }
                            _ => {
                                println!("Received unknown or unhandled data.");
                            }
                        }
                    },
                    Err(_) => {
                        let _ = tauri_handle.lock().unwrap().emit_all("client_disconnected", username_clone);
                        println!("Disconnected!");
                        break;
                    }
                }
                }
                *disconnected.lock().unwrap() = true;
        });

        // std::thread::spawn(move || {
        //     loop {
        //         let mut locked_stream = stream_clone.lock().unwrap();
        //         let rst = read_buffer(&mut locked_stream);
        //         match rst {
        //             Ok(ref array) => {
        //                 let parsed_text = String::from_utf8_lossy(array.as_slice());


        //                 for t in &lines {
        //                     let text = t.trim().to_string();

        //                     if text.starts_with("file_manager||") {
        //                         let arr: Vec<&str> = text.split("||").collect();
        //                         files.lock().unwrap().push(format!("{}||{}", arr[2], arr[1]))
        //                     }

        //                     if text.starts_with("disks||") {
        //                         let arr: Vec<&str> = text.split("||").collect();
        //                         files.lock().unwrap().push(format!("{}||dir", arr[1]));
        //                     }

        //                     if text.starts_with("current_folder||") {
        //                         let arr: Vec<&str> = text.split("||").collect();
        //                         *folder_path.lock().unwrap() = arr[1].to_string();
        //                     }

        //                     if text.starts_with("downloadedfile||") {
        //                         let arr: Vec<&str> = text.split("||").collect();
        //                         let file_name = arr[1];
        //                         let len = 16 + file_name.len() + 2;
        //                         let file = &array[len..array.len()];

        //                         let _ = std::fs::write(file_name, file);
        //                     }

        //                     if text.starts_with("screenshot||") {
        //                         let img = &array[12..array.len()];
        //                         let vec_img: Vec<u8> = img.to_vec();
                                
        //                         let base64_img = general_purpose::STANDARD.encode(vec_img);

        //                         let _ = tauri_handle.lock().unwrap().emit_all("client_screenshot", base64_img.clone());
        //                     }

        //                     if text.starts_with("processes||") {
        //                         let arr: Vec<&str> = text.split("processes||").collect();
        //                         *process_list.lock().unwrap() = arr[1].to_string();
        //                     }
        //                 }
        //             },
        //             Err(_) => {
        //                 let _ = tauri_handle.lock().unwrap().emit_all("client_disconnected", username_clone);
        //                 println!("Disconnected!");
        //                 break;
        //             }
        //         }
        //     }
        //     *disconnected.lock().unwrap() = true;
        // });
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_hostname(&self) -> String {
        self.hostname.clone()
    }

    pub fn get_os(&self) -> String {
        self.os.clone()
    }

    pub fn get_ram(&self) -> String {
        self.ram.clone()
    }

    pub fn get_cpu(&self) -> String {
        self.cpu.clone()
    }

    pub fn get_gpus(&self) -> Vec<String> {
        self.gpus.clone()
    }

    pub fn get_storage(&self) -> Vec<String> {
        self.storage.clone()
    }

    pub fn get_displays(&self) -> i32 {
        self.displays
    }

    pub fn get_ip(&self) -> String {
        self.ip.clone()
    }

    pub fn is_elevated(&self) -> bool {
        self.is_elevated
    }

    pub fn is_disconnect(&self) -> bool {
        *self.disconnected.lock().unwrap()
    }
}

// fn read_buffer(stream: Arc<Mutex<TcpStream>>) -> std::io::Result<Vec<u8>> {
//     let mut stream = stream.lock().unwrap();
//     let mut size_bytes = [0_u8; 4];
//     stream.read_exact(&mut size_bytes)?;

//     let size = u32::from_be_bytes(size_bytes) as usize;
//     let mut buffer = vec![0_u8; size];
//     stream.read_exact(&mut buffer)?;

//     Ok(buffer)
// }