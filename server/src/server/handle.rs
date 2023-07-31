use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, TcpStream, TcpListener, Shutdown};
use std::thread;
use std::io::{Read, Write};
use std::str::from_utf8;

use crate::message::*;


pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0 as u8; 50];

    while match stream.read(&mut buffer) {
        Ok(size) => {
            stream.write(&buffer[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("[CLIENT] Connection closed {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {
        println!("[CLIENT] {}", from_utf8(&buffer).unwrap());
        stream.shutdown(Shutdown::Both).unwrap();
    }


}

pub fn stream_listen(socket: SocketAddr) {
    let listener = TcpListener::bind(&socket).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());

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