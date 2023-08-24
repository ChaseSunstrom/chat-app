use std::env;
use crate::input::get_input;
use crate::server_connect::connect;


mod server_connect;
mod input;
mod message;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            let ip = args[1].parse::<String>().unwrap();

            let port = args[2].parse::<u16>().unwrap();

            connect(ip, port);
        }

        _ => {
            connect(None, None);
        }
    }
}
