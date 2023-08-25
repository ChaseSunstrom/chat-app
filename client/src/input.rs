pub fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn parse_input(ip: Option<String>, port: Option<u16>) -> (Vec<u8>, u16) {
    match (ip, port) {
        (Some(ip), Some(port)) => {
            let ip = ip.split('.')
                .map(|x| x.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();

            (ip, port)
        },

        _ => {
            (vec![127, 0, 0, 1], 8080)
        }
    }
}