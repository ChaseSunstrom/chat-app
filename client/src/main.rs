use std::env;
use crate::input::{get_input, parse_input};
use crate::server_connect::connect;


mod server_connect;
mod input;
mod message;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (ip, port) = parse_input(
        Some(args[1].parse::<String>().unwrap()),
        Some(args[2].parse::<u16>().unwrap()));

    connect(ip, port);
}
