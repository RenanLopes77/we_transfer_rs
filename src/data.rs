use std::{
    fs::File,
    io::{self, BufRead, Write},
    net::TcpStream,
};

pub fn send_data(stream: &mut TcpStream, data: &Vec<u8>) {
    stream.write(data).expect("Unable to send data.");
}

pub fn receive_data(stream: &mut TcpStream) -> Vec<u8> {
    let mut reader = io::BufReader::new(stream);

    let received: Vec<u8> = reader.fill_buf().expect("Unsable to read data.").to_vec();

    assert_ne!(received.len(), 0, "MENSAGEM VAZIA");

    let header_size = received[0] as usize;
    let file_name = String::from_utf8(received[1..header_size].to_vec()).unwrap();

    println!("PRIMEIRO BYTE {:?}", header_size);
    println!("NOME DO ARQUIVO {:?}", file_name);

    let mut file = File::create(format!("./Downloads/{file_name}"))
        .expect("Error encountered while creating file!");
    file.write_all(&received[header_size..received.len()])
        .expect("Error while writing to file");

    file.sync_all().unwrap();

    reader.consume(received.len());

    received
}
