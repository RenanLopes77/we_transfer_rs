use env_args::{get_address, get_mode, get_path, Mode};
use server::connect_server;

use crate::client::connect_client;
use crate::data::{receive_data, send_data};
use crate::files::load_file;

pub mod client;
pub mod constants;
pub mod data;
pub mod env_args;
pub mod files;
pub mod server;

// PESQUISAR SOBRE ALOCAÇÃO DE ESPAÇO (PEDIR ESPAÇO PARA O SISTEMA DE ARQUIVOS)

// OBJETIVOS:
// DESBLOQUEAR TAMANHO DO DOWNLOAD
// BARRINHA DE DOWNLOAD
//  OBTER TAMANHO TOTAL
//  OBTER TAMANHO BAIXADO

fn main() {
    match get_mode() {
        Mode::Server => server_connection(),
        Mode::Client => client_connection(),
    };
}

fn server_connection() {
    let listener = connect_server();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        receive_data(&mut stream);
        // println!("Data received: {:?}", String::from_utf8(data).unwrap());
    }
}

fn client_connection() {
    let mut stream = connect_client(&get_address());
    send_data(&mut stream, &load_file(&get_path()));
}
