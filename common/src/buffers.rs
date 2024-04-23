use std::io::{Read, Write};
use std::net::TcpStream;
use serde::{Serialize, Deserialize};
use rmp_serde::Serializer;
use std::result::Result;
use crate::commands::Command;

pub fn read_buffer(stream: &mut TcpStream) -> Result<Command, Box<dyn std::error::Error>> {
    let mut size_buf = [0_u8; 4];
    stream.read_exact(&mut size_buf)?;
    
    let size = u32::from_be_bytes(size_buf) as usize;

    let mut data_buf = vec![0u8; size];
    stream.read_exact(&mut data_buf)?;

    let command = rmp_serde::from_slice(&data_buf)?;
    Ok(command)
}



pub fn write_buffer(stream: &mut TcpStream, command: Command) {
    let mut buf = Vec::new();
    command.serialize(&mut Serializer::new(&mut buf)).unwrap();

    let _ = stream.write_all(&(buf.len() as u32).to_be_bytes());
    let _ = stream.write_all(&buf);
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


#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptionRequest {
    pub token: [u8; 32],
    pub pub_key: Vec<u8>,
}