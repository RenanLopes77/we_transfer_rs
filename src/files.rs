use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::PathBuf,
};

pub fn load_file(path: &String) -> Vec<u8> {
    let srcdir = PathBuf::from(format!("./{}", path));
    let full_path = fs::canonicalize(&srcdir).unwrap();

    println!("FileName: {:?}", full_path.file_name());
    println!("Extension: {:?}", full_path.extension());
    println!("{}", full_path.display());

    // let file = File::open(file_name).expect("Não foi possível abrir o arquivo.");

    let file = File::open(&full_path).unwrap();
    let mut reader = io::BufReader::new(file);

    let mut received: Vec<u8> = BufRead::fill_buf(&mut reader)
        .expect("Não consigo ler nada.")
        .to_vec();
    reader.consume(received.len());

    let file_name = full_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .as_bytes()
        .to_vec();
    let mut file_header: Vec<u8> = vec![0; file_name.len()];
    file_header[0] = (file_name.len() + 1) as u8;

    file_header.splice(1..file_name.len(), file_name);
    file_header.append(&mut received);

    return file_header;
}
