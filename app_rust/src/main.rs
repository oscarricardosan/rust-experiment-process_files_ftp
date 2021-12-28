mod config_app;

use std::{fs, io};
use std::io::Write;
use ftp_client::prelude::Client;
use chrono::Local;
use crate::config_app::ConfigApp;


fn main() {
    let mut config_app = ConfigApp::new();

    if !config_app.is_configured(){
        config_app.require_config_data();
    }

    let mut client = Client::connect(&config_app.ftp_url, &config_app.ftp_user, &config_app.ftp_password).unwrap();
    let files = client.list_names(&config_app.directory_guias_cpm).unwrap();
    println!("=> Se inicia procesamiento de {} archivo(s): ", files.len());
    for (index, file_origin_path) in files.iter().enumerate() {
        print!(" * {}) {} Procesando archivo {}. ", index, Local::now().format("%Y-%m-%d %H:%M:%S") ,file_origin_path);
        io::stdout().flush().unwrap();

        client.binary().unwrap();

        match client.retrieve_file(&file_origin_path) {
            Ok(retr)=>{
                let part_name = file_origin_path.split('/');
                let destination = format!("./{}/{}", "tmp", part_name.last().unwrap());
                fs::write(&destination, retr).unwrap();
                println!("Guardado en {}", destination);
            }
            Err(error)=> {
                println!(
                    "Error {}. <Posible causa del error: el archivo es una carpeta>",
                    error.to_string()
                );
            }
        }
    }
    println!("=> Proceso finalizado");

}