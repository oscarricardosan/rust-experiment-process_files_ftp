extern crate ftp;

use std::{fs};
use chrono::Local;
use ftp::{FtpStream};
use ftp::types::{FileType};

fn main() {
    let mut ftp = FtpStream::connect("127.0.0.1:21").unwrap();
    let _ = ftp.login("savne", "savne").unwrap_or_else(|_e|{
        panic!("Error de Usuario o contraseña");
    });
    ftp.transfer_type(FileType::Binary).unwrap();

    let files= ftp.nlst(Some("/"));

    for (index, file_name) in files.as_ref().unwrap().iter().enumerate(){
        print!("{}) {} Procesando archivo {}. ", index, Local::now().format("%Y-%m-%d %H:%M:%S") ,file_name);

        match ftp.simple_retr(file_name) {
            Ok(retr)=>{
                fs::write(format!("{}/{}", "tmp", file_name), retr.into_inner()).unwrap();
                println!("Finalizado exitosamente.");
            }
            Err(error)=> {
                println!(
                    "Error al descargar archivo {}: {}   <Posible causa del error: el archivo es una carpeta>",
                    file_name, error.to_string()
                );
            }
        }
    }
}