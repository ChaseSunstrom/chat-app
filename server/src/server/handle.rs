use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, TcpStream, TcpListener, Shutdown};
use std::thread;
use std::io::{Read, Write};
use std::str::from_utf8;

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0 as u8; 100];

    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    // Connection closed by the client
                    break;
                }
                let message = from_utf8(&buffer[0..size]).unwrap();
                println!("[CLIENT] {}", message);

                // Echo back the message
                stream.write(&buffer[0..size]).unwrap();
            },
            Err(_) => {
                // Connection error occurred
                break;
            }
        }
    }

    stream.shutdown(Shutdown::Both).unwrap();
}

pub fn stream_listen(socket: SocketAddr) {
    let listener = TcpListener::bind(&socket).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
