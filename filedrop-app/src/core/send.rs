use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::core::types::TransferState;

pub fn send_file(path: String, addr: String, state: Arc<Mutex<TransferState>>) {
    thread::spawn(move || {
        let mut s = state.lock().unwrap();
        s.filename = path.clone();
        drop(s);

        let mut file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                state.lock().unwrap().error = Some(format!("Open error: {}", e));
                return;
            }
        };

        let metadata = file.metadata().unwrap();
        let size = metadata.len();

        {
            let mut s = state.lock().unwrap();
            s.filesize = size;
        }

        let mut stream = match TcpStream::connect(addr) {
            Ok(s) => s,
            Err(e) => {
                state.lock().unwrap().error = Some(format!("Connect error: {}", e));
                return;
            }
        };

        let filename = std::path::Path::new(&path).file_name().unwrap().to_string_lossy();
        writeln!(stream, "{}:{}", filename, size).unwrap();

        let mut buffer = [0u8; 4096];
        let mut sent = 0u64;

        loop {
            let n = file.read(&mut buffer).unwrap();
            if n == 0 { break; }
            stream.write_all(&buffer[..n]).unwrap();
            sent += n as u64;

            let mut s = state.lock().unwrap();
            s.transferred = sent;
        }

        state.lock().unwrap().complete = true;
    });
}
