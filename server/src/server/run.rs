use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener};
use std::thread;


mod handle;

pub fn default() {
    let socket = SocketAddr::V4(
        SocketAddrV4::new(
            Ipv4Addr::new(127, 0, 0, 1),
            8080
        )
    );

    thread::spawn(move || {
        handle::stream_listen(socket);
    });

    /*thread::spawn(move || {
        connect::send_message(socket, String::from("Hello, World!"));
    });*/
}

pub fn run(ip: String, port: u16) {

    let split_ip = ip.split('.')
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
    );

    handle::stream_listen(socket);


}