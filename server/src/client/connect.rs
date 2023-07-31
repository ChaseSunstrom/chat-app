use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::str::from_utf8;
use crate::message::{Message, User};


pub fn send_message(socket: SocketAddr, msg: String) {

    const BUFFER_SIZE: usize = 12;

    match TcpStream::connect(socket) {
        Ok(mut stream) => {
            //bincode::serialize(&message).unwrap();

            stream.write(msg.as_bytes()).unwrap();

            let mut buffer = [0 as u8; BUFFER_SIZE];

            match stream.read_exact(&mut buffer) {
                Ok(_) => {
                    if &buffer == msg.as_bytes() {
                        println!("[CLIENT] Message sent. {}", msg);
                    } else {
                        let text = from_utf8(&buffer).unwrap();
                        println!("[SERVER] Failed to recieve message: {}\
                                           Got instead: {}", msg, text);
                    }
                },
                Err(e) => {
                    println!("[SERVER] Failed to recieve message: {}", e);
                }
            }
        },
        Err(e) => {
            println!("[SERVER] Failed to connect: {}", e);
        }
    }

    println!("Connection closed.");
}