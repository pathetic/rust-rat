use std::net::TcpStream;
use std::path::{ PathBuf, Path };

use common::buffers::write_buffer;
use common::commands::{ Command, File, FileData };

pub struct FileManager {
    pub current_path: PathBuf,
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            current_path: PathBuf::new(),
        }
    }

    pub fn list_available_disks(&self, write_stream: &mut TcpStream, secret: &Option<Vec<u8>>) {
        write_buffer(
            write_stream,
            Command::DisksResult(get_available_disks()),
            secret,
            crate::NONCE_WRITE.lock().unwrap().as_mut()
        );
    }

    pub fn write_current_folder(&self, write_stream: &mut TcpStream, secret: &Option<Vec<u8>>) {
        write_buffer(
            write_stream,
            Command::CurrentFolder(self.current_path.to_string_lossy().to_string()),
            secret,
            crate::NONCE_WRITE.lock().unwrap().as_mut()
        );
    }

    pub fn view_folder(
        &mut self,
        write_stream: &mut TcpStream,
        folder: &str,
        secret: &Option<Vec<u8>>
    ) {
        self.current_path.push(folder);
        self.list_directory_contents(write_stream, secret);
        self.write_current_folder(write_stream, secret);
    }

    pub fn navigate_to_parent(&mut self, write_stream: &mut TcpStream, secret: &Option<Vec<u8>>) {
        if let Some(parent) = self.current_path.parent() {
            self.current_path = parent.to_path_buf();
            self.list_directory_contents(write_stream, secret);
        } else {
            self.list_available_disks(write_stream, secret);
        }
        self.write_current_folder(write_stream, secret);
    }

    pub fn remove_directory(
        &mut self,
        write_stream: &mut TcpStream,
        directory: &str,
        secret: &Option<Vec<u8>>
    ) {
        let dir_path = self.current_path.join(directory);
        if std::fs::remove_dir_all(dir_path).is_ok() {
            self.list_directory_contents(write_stream, secret);
        }
    }

    pub fn remove_file(
        &mut self,
        write_stream: &mut TcpStream,
        file: &str,
        secret: &Option<Vec<u8>>
    ) {
        let file_path = self.current_path.join(file);
        if std::fs::remove_file(file_path).is_ok() {
            self.list_directory_contents(write_stream, secret);
        }
    }

    pub fn list_directory_contents(&self, write_stream: &mut TcpStream, secret: &Option<Vec<u8>>) {
        if let Ok(entries) = self.current_path.read_dir() {
            let mut file_entries: Vec<File> = Vec::new();
            for entry in entries.filter_map(Result::ok) {
                match entry.file_type() {
                    Ok(file_type) => {
                        let name = entry.file_name().to_string_lossy().to_string();
                        if file_type.is_dir() {
                            file_entries.push(File {
                                file_type: "dir".to_string(),
                                name: name.clone(),
                            });
                        } else if file_type.is_file() {
                            file_entries.push(File {
                                file_type: "file".to_string(),
                                name: name.clone(),
                            });
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read file type: {}", e);
                    }
                }
            }
            write_buffer(
                write_stream,
                Command::FileList(file_entries),
                secret,
                crate::NONCE_WRITE.lock().unwrap().as_mut()
            );
        } else {
            eprintln!("Could not read directory: {}", self.current_path.display());
        }
    }

    pub fn download_file(
        &self,
        write_stream: &mut TcpStream,
        filename: &str,
        secret: &Option<Vec<u8>>
    ) {
        let file_path = self.current_path.join(filename);
        if let Ok(data) = std::fs::read(&file_path) {
            write_buffer(
                write_stream,
                Command::DonwloadFileResult(FileData { name: filename.to_string(), data }),
                secret,
                crate::NONCE_WRITE.lock().unwrap().as_mut()
            );
        } else {
            eprintln!("Failed to read file: {}", file_path.display());
        }
    }
}

// pub fn file_manager(
//     write_stream: &mut TcpStream,
//     current_path: &mut PathBuf,
//     command: &str,
//     path: &str,
//     secret: &Option<Vec<u8>>
// ) {
//     match command {
//         "AVAILABLE_DISKS" => list_available_disks(write_stream, secret),
//         "PREVIOUS_DIR" => navigate_to_parent(write_stream, current_path, secret),
//         "VIEW_DIR" => view_folder(write_stream, current_path, path, secret),
//         "REMOVE_DIR" => remove_directory(write_stream, current_path, path, secret),
//         "REMOVE_FILE" => remove_file(write_stream, current_path, path, secret),
//         "DOWNLOAD_FILE" => download_file(write_stream, current_path, path, secret),
//         _ => {}
//     }
// }

fn get_available_disks() -> Vec<String> {
    let arr = [
        "A",
        "B",
        "C",
        "D",
        "E",
        "F",
        "G",
        "H",
        "I",
        "J",
        "K",
        "L",
        "M",
        "N",
        "O",
        "P",
        "Q",
        "R",
        "S",
        "T",
        "U",
        "V",
        "W",
        "X",
        "Y",
        "Z",
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
