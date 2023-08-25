use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream, Ipv4Addr, SocketAddrV4, IpAddr};
use std::str::from_utf8;
use crate::input::*;

pub fn connect(ip: Vec<u8>, port: u16) {
    let socket = SocketAddr::V4(
        SocketAddrV4::new(
            Ipv4Addr::new(
                ip[0],
                ip[1],
                ip[2],
                ip[3],
            ),
            port
        )
    );

    match TcpStream::connect(socket) {
        Ok(mut stream) => {
            loop {
                let mut msg = get_input();

                const BUFFER_SIZE: usize = 100;

                if msg.len() < BUFFER_SIZE {
                    while msg.len() < BUFFER_SIZE {
                        msg.push_str(" ");
                    }
                } else if msg.len() > BUFFER_SIZE {
                    msg.truncate(BUFFER_SIZE);
                }

                stream.write(msg.as_bytes()).unwrap();

                let mut buffer = [0; BUFFER_SIZE];

                match stream.read_exact(&mut buffer) {
                    Ok(_) => {
                        if &buffer[0..BUFFER_SIZE] == msg.as_bytes() {
                            println!("[SERVER] Message sent: {}", msg);
                        } else {
                            let text = from_utf8(&buffer).unwrap();
                            println!(
                                "[SERVER] Failed to receive message: {}\nGot instead: {}",
                                msg, text
                            );
                        }
                    }
                    Err(e) => {
                        println!("[SERVER] Failed to receive message: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("[SERVER] Failed to connect: {}", e);
        }
    }
}



