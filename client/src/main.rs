use std::env;
use crate::input::get_input;


mod server_connect;
mod input;
mod message;

fn main() {
    server_connect::connect();
}
