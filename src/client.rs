use std::net::TcpStream;

pub fn connect_client(address: &String) -> TcpStream {
    match TcpStream::connect(address) {
        Ok(stream) => {
            println!("\n✅ Connected Successfully ✅\n");
            return stream;
        }
        // Err(err) => panic!("💀 Was not possible to connect with the server 💀 - {err}"),
        Err(err) => panic!("💀 {}", err),
    }
}
