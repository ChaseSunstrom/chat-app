use std::env;


mod server_connect;
mod input;

fn main() {
    //let args: Vec<String> = env::args().collect();

    /*match args.len() {
        3 => {
            let ip = args[1].parse::<String>().unwrap();

            let port = args[2].parse::<u16>().unwrap();

            server_connect::connect(ip, port);
        }

        _ => {

        }
    }*/

    server_connect::connect();
}
