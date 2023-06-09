use std::net::TcpListener;

use crate::constants::SERVER_ADRESS;

pub fn connect_server() -> TcpListener {
    match TcpListener::bind(SERVER_ADRESS) {
        Ok(listener) => {
            println!("\n✅ SERVER WORKING ✅\n");
            return listener;
        }
        Err(err) => panic!("💀 SERVER NOT NORKING 💀  - {}", err),
    }
}
