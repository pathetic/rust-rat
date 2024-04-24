use std::io::{ Read, Write };
use std::net::TcpStream;
use serde::{ Serialize, Deserialize };
use rmp_serde::Serializer;
use std::result::Result;
use crate::commands::Command;
use rand_chacha::ChaCha20Rng;

use chacha20poly1305::{
    aead::{Aead, NewAead},
    XChaCha20Poly1305,
};

pub fn read_buffer(stream: &mut TcpStream, secret: &Option<Vec<u8>>) -> Result<Command, Box<dyn std::error::Error>> {
    match secret {
        Some(secret) => {
            let mut size_buf = [0_u8; 4];
            stream.read_exact(&mut size_buf)?;

            let size = u32::from_be_bytes(size_buf) as usize;

            let mut data_buf = vec![0u8; size];
            stream.read_exact(&mut data_buf)?;

            let slice = secret.as_slice();
            let secret_u832: Result<&[u8; 32], _> = slice.try_into();

            let buffer = decrypt_buffer(&data_buf, secret_u832.unwrap(), &[0; 24]);

            let packet: Packet = rmp_serde::from_slice(&buffer)?;

            println!("packet command: {:?}", packet.command);

            Ok(packet.command)
        },
        None => {
            let mut size_buf = [0_u8; 4];
            stream.read_exact(&mut size_buf)?;
        
            let size = u32::from_be_bytes(size_buf) as usize;
        
            let mut data_buf = vec![0u8; size];
            stream.read_exact(&mut data_buf)?;
        
            let packet: Packet = rmp_serde::from_slice(&data_buf)?;
        
            println!("packet command: {:?}", packet.command);
            println!("packet test data: {}", packet.test_data);
        
            Ok(packet.command)
        }
    }
}

pub fn write_buffer(stream: &mut TcpStream, command: Command, secret: &Option<Vec<u8>>) {
    match secret {
        Some(secret) => {
            let mut buf = Vec::new();

            let packet = Packet { command, test_data: "test".to_string() };
        
            packet.serialize(&mut Serializer::new(&mut buf)).unwrap();vec_to_array32(secret).unwrap();

            let buffer = encrypt_buffer(&buf, vec_to_array32(secret).unwrap(), &[0; 24]);

            let _ = stream.write_all(&(buffer.len() as u32).to_be_bytes());
            let _ = stream.write_all(&buffer);
        },
        None => {
            let mut buf = Vec::new();

            let packet = Packet { command, test_data: "test".to_string() };
        
            packet.serialize(&mut Serializer::new(&mut buf)).unwrap();
        
            let _ = stream.write_all(&(buf.len() as u32).to_be_bytes());
            let _ = stream.write_all(&buf);
        }
    }

}

fn vec_to_array32(bytes: &Vec<u8>) -> Result<&[u8; 32], String> {
    if bytes.len() == 32 {
        let slice = bytes.as_slice();
        let array_ref: Result<&[u8; 32], _> = slice.try_into();
        array_ref.map_err(|_| "Conversion failed due to incorrect length".to_string())
    } else {
        Err("Vector does not contain exactly 32 bytes".to_string())
    }
}

pub fn encrypt_buffer(
    buffer: &[u8],
    secret: &[u8; crate::SECRET_LEN],
    nonce: &[u8; crate::NONCE_LEN],
) -> Vec<u8> {
    let cipher = XChaCha20Poly1305::new(secret.into());
    let buf = cipher.encrypt(nonce.into(), buffer.as_ref()).unwrap();

    buf
}

pub fn decrypt_buffer(
    buffer: &[u8],
    secret: &[u8; crate::SECRET_LEN],
    nonce: &[u8; crate::NONCE_LEN],
) -> Vec<u8> {
    let cipher = XChaCha20Poly1305::new(secret.into());
    let buf = cipher.decrypt(nonce.into(), buffer.as_ref()).unwrap();

    buf
}

pub fn read_console_buffer<I>(stream: &mut I) -> core::result::Result<Vec<u8>, ()> where I: Read {
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
        }
        Err(_) => { Err(()) }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {
    pub command: Command,
    pub test_data: String,
}