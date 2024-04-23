use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};

use crate::client;
use common::buffers::{read_buffer, write_buffer};

use rand::rngs::OsRng;
use rand::Rng;
use rand::RngCore;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use rsa::{traits::PaddingScheme, RsaPrivateKey, RsaPublicKey};
use anyhow::Context;
use rsa::pkcs8::EncodePublicKey;
use base64::{engine::general_purpose, Engine as _};

use common::commands::{Command, ReceiveClient, File};


fn encryption_request(mut stream: TcpStream, pub_key: Vec<u8>) -> String {
    let init_msg = "encryption_request||";
    let mut token = [0u8; common::ENC_TOK_LEN];
    OsRng.fill(&mut token);

    let token = general_purpose::STANDARD.encode(token);
    let pub_key = general_purpose::STANDARD.encode(pub_key);

    let string = init_msg.to_string() + &token + "||" + &pub_key;

    let size = string.len() as u32;
    let size_bytes = size.to_be_bytes();

    stream.write_all(&size_bytes).unwrap();
    stream.write_all(string.as_bytes()).unwrap();

    token
}

fn get_client(mut stream: TcpStream) -> ReceiveClient {
    write_buffer(&mut stream, Command::InitClient);

    let rcv = read_buffer(&mut stream.try_clone().unwrap()).unwrap();

    match rcv {
        Command::Client(client) => client,
        _ => panic!("Invalid command received"),
    }
}

#[derive(Clone)]
pub struct Server {
    pub tauri_handle: Arc<Mutex<AppHandle>>,
    pub clients: Arc<Mutex<Vec<client::Client>>>,

    private_key: rsa::RsaPrivateKey,
    public_key: rsa::RsaPublicKey,

    files: Arc<Mutex<Vec<File>>>,
    current_path: Arc<Mutex<String>>,
}

impl Server {
    pub fn new(app_handle: AppHandle) -> Self {
        let mut rng = OsRng;

        let priv_key =
        RsaPrivateKey::new(&mut rng, common::RSA_BITS).with_context(|| "Failed to generate a key.").unwrap();
        let pub_key = RsaPublicKey::from(&priv_key);

        Server {
            tauri_handle: Arc::new(Mutex::new(app_handle)),
            clients: Arc::new(Mutex::new(Vec::new())),
            private_key: priv_key,
            public_key: pub_key,
            files: Arc::new(Mutex::new(Vec::new())),
            current_path: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn get_files (&self) -> Vec<File> {
        self.files.lock().unwrap().clone()
    }

    pub fn reset_files (&mut self) {
        self.files.lock().unwrap().clear();
    }

    pub fn get_folder_path (&self) -> String {
        self.current_path.lock().unwrap().clone()
    }

    pub fn reset_folder_path (&mut self) {
        self.current_path.lock().unwrap().clear();
    }

    pub fn listen_port(&mut self, port: String) -> bool {
        let listener = TcpListener::bind("0.0.0.0:".to_string() + port.as_str());

        if let Ok(stream) = listener {
            let vec = Arc::clone(&self.clients);
            let files = Arc::clone(&self.files);
            let current_path = Arc::clone(&self.current_path);
            let tauri_handle = Arc::clone(&self.tauri_handle);
            let public_key = self.public_key.clone();


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

                    //let encryption_request = encryption_request(stream.try_clone().unwrap(), public_key.to_public_key_der().unwrap().as_ref().to_vec());

                    let info = get_client(stream.try_clone().unwrap());

                    let client = client::Client::new(
                        Arc::clone(&tauri_handle),
                        stream.try_clone().unwrap(),
                        info.username.clone(),
                        info.hostname,
                        info.os,
                        info.ram,
                        info.cpu,
                        info.gpus,
                        info.storage,
                        info.displays,
                        ip,
                        info.is_elevated,
                        Arc::clone(&files),
                        Arc::clone(&current_path),
                    );

                    let _ = tauri_handle.lock().unwrap().emit_all("client_connected", info.username);

                    vec.lock().unwrap().push(client);
                }
            });
            true
        } else {
            false
        }
    }
}