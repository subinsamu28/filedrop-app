use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::core::types::TransferState;

pub fn receive_file(port: u16, state: Arc<Mutex<TransferState>>) {
    thread::spawn(move || {
        let listener = TcpListener::bind(("0.0.0.0", port)).expect("Bind failed");
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut reader = BufReader::new(&mut stream);

            let mut header = String::new();
            reader.read_line(&mut header).unwrap();
            let parts: Vec<&str> = header.trim().split(':').collect();
            let filename = parts[0].to_string();
            let filesize: u64 = parts[1].parse().unwrap();

            {
                let mut s = state.lock().unwrap();
                s.filename = filename.clone();
                s.filesize = filesize;
            }

            let mut output = File::create(&filename).unwrap();
            let mut received = 0;
            let mut buffer = [0u8; 4096];

            while received < filesize {
                let n = reader.read(&mut buffer).unwrap();
                if n == 0 { break; }
                output.write_all(&buffer[..n]).unwrap();
                received += n as u64;

                let mut s = state.lock().unwrap();
                s.transferred = received;
            }

            state.lock().unwrap().complete = true;
            break;
        }
    });
}
