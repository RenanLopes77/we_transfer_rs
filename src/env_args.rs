use std::env;

pub enum Mode {
    Client,
    Server,
}

pub fn get_mode() -> Mode {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(arg) if arg.to_lowercase() == "s" => Mode::Server,
        Some(arg) if arg.to_lowercase() == "c" => Mode::Client,
        Some(arg) => {
            panic!("{} is not a valid argument. Send \"c\" for client or \"s\" for server, eg. \"cargo run c\"", arg);
        }
        None => panic!(
            "No argument received. Send \"c\" for client or \"s\" for server, eg. \"cargo run c\""
        ),
    }
}

pub fn get_address() -> String {
    let args: Vec<String> = env::args().collect();
    match args.get(2) {
        Some(address) => {
            return address.to_string();
        }
        None => panic!(
            "No argument received. Send \"c\" for client or \"s\" for server, eg. \"cargo run c\""
        ),
    }
}

pub fn get_path() -> String {
    let args: Vec<String> = env::args().collect();
    match args.get(3) {
        Some(path) => {
            return path.to_string();
        }
        None => panic!(
            "No argument received. Send \"c\" for client or \"s\" for server, eg. \"cargo run c\""
        ),
    }
}
