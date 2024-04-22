use std::net::TcpStream;
use sysinfo::{System, Pid};
use common::buffers::write_bytes;

pub fn process_list(write_stream: &mut TcpStream) {
    let mut s = System::new_all();

    s.refresh_all();

    let mut processes = Vec::new();

    for (pid, process) in s.processes() {
        processes.push(format!("{}||{}", pid, process.name()));
        let procs = processes.join(",");
        let header = "processes||".to_string();
        write_bytes(write_stream, format!("{}{}", header, procs).as_bytes());
    }
}

pub fn kill_process(pid: usize) {
    let mut s = System::new_all();

    s.refresh_all();

    if let Some(process) = s.process(Pid::from(pid)) {
        process.kill();
    }
}