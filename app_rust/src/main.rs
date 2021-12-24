use std::{fs};
use ftp_client::prelude::Client;
use chrono::Local;

fn main() -> Result<(), ftp_client::error::Error> {
    let mut client = Client::connect("xxx", "xxx", "xx")?;
    let files = client.list_names("/")?;
    println!("Listando archivos: ");
    for (index, file_origin_path) in files.iter().enumerate() {
        print!("{}) {} Procesando archivo {}. ", index, Local::now().format("%Y-%m-%d %H:%M:%S") ,file_origin_path);
        client.binary().unwrap();

        match client.retrieve_file(&file_origin_path) {
            Ok(retr)=>{
                let part_name = file_origin_path.split('/');
                let destination = format!("./{}/{}", "tmp", part_name.last().unwrap());
                fs::write(&destination, retr).unwrap();
                println!("Finalizado exitosamente. Se guarda archivo en {}", destination);
            }
            Err(error)=> {
                println!(
                    "Error al descargar archivo {}: {}   <Posible causa del error: el archivo es una carpeta>",
                    file_origin_path, error.to_string()
                );
            }
        }
    }
    Ok(())

}