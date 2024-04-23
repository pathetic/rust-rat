use std::io::{Read, Write, Result};
use std::net::TcpStream;
use serde::{Serialize, Deserialize};

pub fn read_buffer(stream: &mut TcpStream) -> Result<Vec<u8>> {
    let mut size_bytes = [0_u8; 4];
    stream.read_exact(&mut size_bytes)?;

    let size = u32::from_be_bytes(size_bytes) as usize;
    let mut buffer = vec![0_u8; size];
    stream.read_exact(&mut buffer)?;

    Ok(buffer)
}

pub fn write_bytes(stream: &mut TcpStream, bytes: &[u8]) {
    let size = bytes.len() as u32;
    let size_bytes = size.to_be_bytes();

    let _ = stream.write_all(&size_bytes);
    let _ = stream.write_all(bytes);
}

pub fn read_console_buffer<I>(stream: &mut I) -> core::result::Result<Vec<u8>, ()>
where I: Read {
let mut buffer = [0_u8; 1024];
match stream.read(&mut buffer) {
    Ok(size) => {
        if size == 0 {
            return Err(());
        }

        let mut vect: Vec<u8> = Vec::new();
        for v in &buffer[0..size] {
            vect.push(*v);
        }

        Ok(vect)
    },
    Err(_) => {
        Err(())
    }
}
}

// #[derive(Serialize, Deserialize, Debug)]
// enum Command {
//     InitClient(InitClient),
//     ReceiveClient(ReceiveClient),
//     SendPing(SendPing),
//     ReceivePing(ReceivePing),
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Packet {
//     size: u32,
//     data: Vec<u8>,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptionRequest {
    pub token: [u8; 32],
    pub pub_key: Vec<u8>,
}