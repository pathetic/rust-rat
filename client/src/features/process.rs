use std::net::TcpStream;
use sysinfo::{ System, Pid };
use common::buffers::write_buffer;
use common::commands::{ ProcessList, Process, Command };

pub fn process_list(write_stream: &mut TcpStream, secret: &Option<Vec<u8>>) {
    let mut s = System::new_all();

    s.refresh_all();

    let mut process_list = ProcessList {
        processes: Vec::new(),
    };

    for (pid, process) in s.processes() {
        let process: Process = Process {
            pid: pid.as_u32() as usize,
            name: process.name().to_string(),
        };

        process_list.processes.push(process);
    }

    let process_command = Command::ProcessList(process_list);

    write_buffer(write_stream, process_command, secret);
}

pub fn kill_process(pid: usize) {
    let mut s = System::new_all();

    s.refresh_all();

    if let Some(process) = s.process(Pid::from(pid)) {
        process.kill();
    }
}
