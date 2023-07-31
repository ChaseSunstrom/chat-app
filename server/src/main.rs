use std::env;

#[path = "server/run.rs"]
pub mod run;

#[path = "common/message.rs"]
pub mod message;

#[path = "client/connect.rs"]
pub mod connect;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            let ip = args[1].parse::<String>().unwrap();

            let port = args[2].parse::<u16>().unwrap();

            run::run(ip, port);
        }

        _ => {
            run::default();
        }
    }
}
