use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream, Ipv4Addr, SocketAddrV4};
use std::str::from_utf8;
use crate::input::get_input;


pub fn connect(/*ip: String, port: u16*/) {
    /*let split_ip = ip.split('.')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    let socket = SocketAddr::V4(
        SocketAddrV4::new(
            Ipv4Addr::new(
                split_ip[0],
                split_ip[1],
                split_ip[2],
                split_ip[3],
            ),
            port
        )
    );*/

    let socket = SocketAddr::V4(
        SocketAddrV4::new(
            Ipv4Addr::new(
                192,
                168,
                1,
                151,
            ),
            8080
        )
    );


    loop {
        let mut msg = get_input();

        const BUFFER_SIZE: usize = 100;

        if msg.len() < BUFFER_SIZE {
            while msg.len() < BUFFER_SIZE {
                msg.push_str(" ");
            }
        } else if msg.len() > BUFFER_SIZE{
            msg.truncate(BUFFER_SIZE);
        }

        match TcpStream::connect(socket) {
            Ok(mut stream) => {
                //bincode::serialize(&message).unwrap();

                stream.write(msg.as_bytes()).unwrap();

                let mut buffer = [0 as u8; BUFFER_SIZE];

                match stream.read_exact(&mut buffer) {
                    Ok(_) => {
                        if &buffer == msg.as_bytes() {
                            println!("[SERVER] Message sent: {}", msg);
                        } else {
                            let text = from_utf8(&buffer).unwrap();
                            println!("[SERVER] Failed to recieve message: {}\n
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
}