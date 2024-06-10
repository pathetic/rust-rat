use std::net::{ TcpListener, TcpStream };
use std::sync::{ Arc, Mutex };
use tauri::{ AppHandle, Manager };

use crate::client::{ self, Client };
use common::buffers::{ read_buffer, write_buffer };
use rand_chacha::ChaCha20Rng;
use rand::SeedableRng;

use rand::rngs::OsRng;
use rsa::{ RsaPrivateKey, RsaPublicKey };
use anyhow::Context;
use rsa::pkcs8::EncodePublicKey;
use rsa::Pkcs1v15Encrypt;

use common::commands::{
    ClientInfo,
    ClientNonce,
    Command,
    EncryptionRequestData,
    EncryptionResponseData,
};

fn encryption_request(mut stream: TcpStream, pub_key: Vec<u8>) -> EncryptionResponseData {
    write_buffer(
        &mut stream,
        Command::EncryptionRequest(EncryptionRequestData { public_key: pub_key.clone() }),
        &None,
        None
    );

    let rcv = read_buffer(&mut stream.try_clone().unwrap(), &None, None).unwrap();

    match rcv {
        Command::EncryptionResponse(response) => response,
        _ => panic!("Invalid command received"),
    }
}

fn get_client(mut stream: TcpStream, secret: Option<Vec<u8>>) -> (ClientInfo, ClientNonce) {
    let mut seed = [0u8; common::SECRET_LEN];
    seed.copy_from_slice(secret.as_ref().unwrap().as_slice());

    let nonce_write = ChaCha20Rng::from_seed(seed);
    let nonce_read = ChaCha20Rng::from_seed(seed);

    write_buffer(&mut stream, Command::InitClient, &secret, Some(&mut nonce_write.clone()));

    let rcv = read_buffer(
        &mut stream.try_clone().unwrap(),
        &secret,
        Some(&mut nonce_read.clone())
    ).unwrap();

    let client_nonce: ClientNonce = ClientNonce {
        nonce_read: Some(nonce_read),
        nonce_write: Some(nonce_write),
    };

    match rcv {
        Command::Client(client) => (client, client_nonce),
        _ => panic!("Invalid command received"),
    }
}

#[derive(Clone)]
pub struct Server {
    pub tauri_handle: Arc<Mutex<AppHandle>>,
    pub clients: Arc<Mutex<Vec<client::Client>>>,

    private_key: rsa::RsaPrivateKey,
    public_key: rsa::RsaPublicKey,
}

impl Server {
    pub fn new(app_handle: AppHandle) -> Self {
        let mut rng = OsRng;

        let priv_key = RsaPrivateKey::new(&mut rng, common::RSA_BITS)
            .with_context(|| "Failed to generate a key.")
            .unwrap();
        let pub_key = RsaPublicKey::from(&priv_key);

        Server {
            tauri_handle: Arc::new(Mutex::new(app_handle)),
            clients: Arc::new(Mutex::new(Vec::new())),
            private_key: priv_key,
            public_key: pub_key,
        }
    }

    pub fn listen_port2(&mut self, port: String) -> bool {
        let listener = TcpListener::bind("0.0.0.0:".to_string() + &port);
    
        if let Ok(listener) = listener {
            let vec = Arc::clone(&self.clients);
            let tauri_handle = Arc::clone(&self.tauri_handle);
            let public_key = self.public_key.clone();
            let private_key = self.private_key.clone();
    
            tokio::task::spawn_blocking(move || {
                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => {
                            let vec = Arc::clone(&vec);
                            let tauri_handle = Arc::clone(&tauri_handle);
                            let public_key = public_key.clone();
                            let private_key = private_key.clone();
    
                            tokio::task::spawn(async move {
                                handle_client_connection(stream, vec, tauri_handle, public_key, private_key).await;
                            });
                        }
                        Err(e) => {
                            println!("Connection failed: {}", e);
                        }
                    }
                }
            });
            true
        } else {
            false
        }
    }
    
    pub fn listen_port(&mut self, port: String) -> bool {
        let listener = TcpListener::bind("0.0.0.0:".to_string() + port.as_str());

        if let Ok(stream) = listener {
            let vec = Arc::clone(&self.clients);
            let tauri_handle = Arc::clone(&self.tauri_handle);
            let public_key = self.public_key.clone();
            let private_key = self.private_key.clone();

            tokio::task::spawn(async move {
                for i in stream.incoming() {
                    println!("New Client!");

                    if i.is_err() {
                        break;
                    }

                    let stream = i.unwrap();
                    let ip = stream.try_clone().unwrap().peer_addr().unwrap().ip().to_string();

                    println!("encryption_request");
                    let encryption = encryption_request(
                        stream.try_clone().unwrap(),
                        public_key.to_public_key_der().unwrap().as_ref().to_vec()
                    );

                    let padding = Pkcs1v15Encrypt::default();
                    let secret_dec = private_key.decrypt(padding, &encryption.secret).unwrap();

                    println!("get_client");
                    let (info, nonce) = get_client(
                        stream.try_clone().unwrap(),
                        Some(secret_dec.clone())
                    );

                    let mut client = client::Client::new(
                        Arc::clone(&tauri_handle),
                        stream.try_clone().unwrap(),
                        secret_dec,
                        nonce.nonce_read.unwrap(),
                        nonce.nonce_write.unwrap(),
                        info.username.clone(),
                        info.hostname,
                        info.os,
                        info.ram,
                        info.cpu,
                        info.gpus,
                        info.storage,
                        info.displays,
                        ip,
                        info.is_elevated
                    );

                    let _ = tauri_handle
                        .lock()
                        .unwrap()
                        .emit_all("client_connected", info.username);

                    println!("Client added");
                    
                    client.handle_client();
                    client.is_handled = true;
                    vec.lock().unwrap().push(client);
                }
            });
            true
        } else {
            false
        }
    }
}

pub async fn handle_client_connection(
    mut stream: TcpStream,
    clients: Arc<Mutex<Vec<client::Client>>>,
    tauri_handle: Arc<Mutex<AppHandle>>,
    public_key: rsa::RsaPublicKey,
    private_key: rsa::RsaPrivateKey,
) {
    println!("New Client!");
    let ip = stream.peer_addr().unwrap().ip().to_string();

    println!("encryption_request");
    let encryption = encryption_request(
        stream.try_clone().unwrap(),
        public_key.to_public_key_der().unwrap().as_ref().to_vec(),
    );

    let padding = rsa::pkcs1v15::Pkcs1v15Encrypt::default();
    let secret_dec = private_key.decrypt(padding, &encryption.secret).unwrap();

    println!("get_client");
    let (info, nonce) = get_client(
        stream.try_clone().unwrap(),
        Some(secret_dec.clone()),
    );

    let mut client = client::Client::new(
        Arc::clone(&tauri_handle),
        stream.try_clone().unwrap(),
        secret_dec,
        nonce.nonce_read.unwrap(),
        nonce.nonce_write.unwrap(),
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
    );

    let _ = tauri_handle
        .lock()
        .unwrap()
        .emit_all("client_connected", info.username);

    println!("Client added");

    client.handle_client();
    client.is_handled = true;
    clients.lock().unwrap().push(client);
}
