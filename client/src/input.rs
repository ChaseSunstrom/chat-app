pub fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn get_name() -> String {
    println!("Enter in a username: ");
    get_input()
}