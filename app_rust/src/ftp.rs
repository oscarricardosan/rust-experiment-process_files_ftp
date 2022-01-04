use std::{fs};
use std::process::exit;
use std::sync::mpsc;
use chrono::Local;
use loading::Loading;
use crate::ConfigApp;

use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use ftp_client::client::Client;
use crate::thread_pool::ThreadPool;

pub fn start_image_processing(config_app:Arc<Mutex<ConfigApp>>) {
    let mut loading = Loading::new();
    loading.start();
    loading.info("Iniciando conexión con FTP");
    loading.end();

    let get_files= ||{
        let mut loading = Loading::new();
        loading.start();
        let config_app_tmp= config_app.lock().unwrap();

        let mut client_ftp = Client::connect(
            &config_app_tmp.ftp_url, &config_app_tmp.ftp_user, &config_app_tmp.ftp_password
        ).unwrap();
        client_ftp.binary().unwrap();
        loading.success("Conexión exitosa con FTP");

        let files = client_ftp.list_names(&config_app_tmp.directory_guias_cpm).unwrap();
        loading.success("Obtención de listado con archivos en FTP exitosa");
        client_ftp.logout().unwrap();
        loading.info(format!("Se inicia procesamiento de {} archivo(s): ", files.len()));
        loading.end();
        println!("\n");
        files
    };

    let files= get_files();

    if files.len() == 0{
        let mut loading = Loading::new();
        loading.start();
        loading.warn(format!("EJECUCIÓN FINALIZADA: No hay archivos para procesar."));
        loading.end();
        return;
    }

    let pool = ThreadPool::new(40).unwrap_or_else(
        |err|{
            println!("Error al generar pool de conexiones: {}", err);
            exit(1)
        }
    );

    let (tx, rx) = mpsc::channel();

    for (index, file_origin_path) in files.iter().enumerate() {
        let real_index= index +1;
        copy_file_to_local(
            &pool, real_index, file_origin_path.clone(),
            config_app.clone(),
            tx.clone()
        );
    }

    let mut total_files_processed:i64= 0;
    for _received in rx {
        total_files_processed= total_files_processed+1;
        if total_files_processed == files.len() as i64{
            break;
        }
    }

    println!("Proceso finalizado");
}

fn copy_file_to_local(
    pool: &ThreadPool, real_index: usize, file_origin_path: String, config_app: Arc<Mutex<ConfigApp>> ,
    sender: Sender<i64>
) {
    pool.execute(move || {

        let get_ftp_credentials=|| {
            let config_app_tmp= config_app.lock().unwrap();
            (
                config_app_tmp.ftp_url.clone(),
                config_app_tmp.ftp_user.clone(),
                config_app_tmp.ftp_password.clone()
            )
        };
        let (url, user, password)= get_ftp_credentials();
        let mut client_ftp = Client::connect(&url, &user, &password).unwrap();
        client_ftp.binary().unwrap();

        let mut loading = Loading::new();
        loading.start();
        match client_ftp.retrieve_file(&file_origin_path) {
            Ok(retr) => {
                let part_name = file_origin_path.split('/');
                let destination = format!("./{}/{}", "tmp", part_name.last().unwrap());
                fs::write(&destination, retr).unwrap();

                loading.success(
                    format!(" * {}) {} Archivo {} guardado en {}.", real_index, Local::now().format("%Y-%m-%d %H:%M:%S"), file_origin_path, destination)
                );
            }
            Err(error) => {
                loading.fail(
                     format!(
                         "Error {}. <Posible causa del error: el archivo es una carpeta>",
                         error.to_string()
                     )
                );
            }
        }
        loading.end();
        sender.send(1 as i64).unwrap();
    });

}