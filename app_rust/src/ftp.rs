use std::{fs};
use chrono::Local;
use ftp_client::prelude::Client;
use loading::Loading;
use crate::ConfigApp;

pub fn start_image_processing(config_app:&ConfigApp) {
    let mut loading = Loading::new();

    loading.start();
    loading.text("Iniciando conexión con FTP");
    let mut client = Client::connect(&config_app.ftp_url, &config_app.ftp_user, &config_app.ftp_password).unwrap();
    loading.success("Conexión exitosa con FTP");

    loading.text("Obteniendo archivos de FTP");
    let files = client.list_names(&config_app.directory_guias_cpm).unwrap();
    loading.success("Obtención de archivos de FTP exitosa");

    loading.info(format!("Se inicia procesamiento de {} archivo(s): ", files.len()));

    for (index, file_origin_path) in files.iter().enumerate() {

        let real_index= index +1;
        loading.text(format!(" * {}) {} Procesando archivo {}. ", real_index, Local::now().format("%Y-%m-%d %H:%M:%S") ,file_origin_path));

        client.binary().unwrap();

        match client.retrieve_file(&file_origin_path) {
            Ok(retr)=>{
                let part_name = file_origin_path.split('/');
                let destination = format!("./{}/{}", "tmp", part_name.last().unwrap());
                fs::write(&destination, retr).unwrap();

                loading.success(
                    format!(" * {}) {} Archivo {} guardado en {}.", real_index, Local::now().format("%Y-%m-%d %H:%M:%S") ,file_origin_path, destination)
                );
            }
            Err(error)=> {
                loading.fail(
                    format!(
                        "Error {}. <Posible causa del error: el archivo es una carpeta>",
                        error.to_string()
                    )
                );
            }
        }
    }

    loading.info("Proceso finalizado exitosamente");
}