use std::{fs};
use std::process::exit;
use std::sync::mpsc;
use chrono::{Local};
use loading::Loading;

use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use std::time::{Instant};
use ftp_client::client::Client;
use crate::config_app::FtpAttributes;
use crate::db_execution_model::DbExecutionModel;
use crate::db_file_model::DbFileModel;
use crate::thread_pool::ThreadPool;

pub struct Ftp {
    pub ftp_client: Arc<Mutex<Client>>,
    pub ftp_atributes: FtpAttributes,
    db_execution_model: DbExecutionModel,
}
impl Ftp {
    pub fn new(ftp_atributes: FtpAttributes) -> Self {
        Self{
            ftp_client: Arc::new(
                Mutex::new(
                    Client::connect(
                        &ftp_atributes.ftp_url, &ftp_atributes.ftp_user, &ftp_atributes.ftp_password
                    ).unwrap()
                )
            ),
            ftp_atributes,
            db_execution_model: DbExecutionModel::new(),
        }
    }


    pub fn start_image_processing(&mut self) {
        let mut loading = Loading::new();
        loading.start();
        loading.info("Iniciando conexión con FTP");
        loading.end();

        let mut loading = Loading::new();
        loading.start();

        let get_files= ||{
            let mut ftp_client= self.ftp_client.lock().unwrap();
            ftp_client.binary().unwrap();
            loading.success("Conexión exitosa con FTP");

            let files = ftp_client.list_names(&self.ftp_atributes.directory_guias_cpm).unwrap();
            loading.success("Obtención de listado con archivos en FTP exitosa");
            loading.info(format!("Se inicia procesamiento de {} archivo(s): ", files.len()));
            loading.end();
            ftp_client.logout().unwrap();
            println!("\n");
            files
        };

        let files= get_files();

        self.db_execution_model.start_of_process( files.len() as i32);

        if files.len() == 0{
            let mut loading = Loading::new();
            loading.start();
            loading.warn(format!("EJECUCIÓN FINALIZADA: No hay archivos para procesar."));
            loading.end();
            return;
        }

        let total_threads = if files.len() < 20 {
            1
        } else if files.len() < 50 {
            5
        } else if files.len() < 100 {
            10
        } else { 20 };

        let pool = ThreadPool::new(total_threads, self.ftp_atributes.clone()).unwrap_or_else(
            |err|{
                println!("Error al generar pool de conexiones: {}", err);
                exit(1)
            }
        );

        let (tx, rx): (Sender<i64>, Receiver<i64>) = mpsc::channel();

        let now = Instant::now();
        for (index, file_origin_path) in files.iter().enumerate() {
            let real_index= index +1;
            self.copy_file_to_local(
                &pool, real_index, file_origin_path.clone(),
                tx.clone()
            );
        }

        let mut total_files_processed:i32= 0;
        for _received in rx {
            total_files_processed= total_files_processed+1;
            if total_files_processed == files.len() as i32{
                break;
            }
            println!("Archivo procesado {} de {}",total_files_processed, total_files_processed);
        }
        self.db_execution_model.end_of_process( total_files_processed);

        println!("Proceso finalizado");

        let elapsed = now.elapsed();
        println!("*******************************************************");
        println!("**                                                   **");
        println!("**            Tiempo transcurrido: {:.2?}          ", elapsed);
        println!("**                                                   **");
        println!("*******************************************************");
    }



    fn copy_file_to_local(
        &mut self,
        pool: &ThreadPool, real_index: usize, file_origin_path: String,
        sender: Sender<i64>
    ) {
        let execution_id= self.db_execution_model.get_execution_id();
        pool.execute(move |ftp_client, connection_db| {
            let mut loading = Loading::new();
            loading.start();
            loading.info(
                format!("{}) {} Inicia procesamiento de archivo {}.", real_index, Local::now().format("%Y-%m-%d %H:%M:%S"), file_origin_path)
            );
            let mut db_file_model= DbFileModel::new(execution_id, connection_db);
            db_file_model.start_of_process(&file_origin_path);
            match ftp_client.lock().unwrap().retrieve_file(&file_origin_path) {
                Ok(retr) => {
                    let part_name = file_origin_path.split('/');
                    let destination = format!("./{}/{}", "tmp", part_name.last().unwrap());
                    fs::write(&destination, retr).unwrap();

                    loading.success(
                        format!("{}) {} Archivo {} guardado en {}.", real_index, Local::now().format("%Y-%m-%d %H:%M:%S"), file_origin_path, destination)
                    );
                }
                Err(error) => {
                    loading.fail(
                        format!(
                            "Error {}. <Posible causa del error: el archivo es una carpeta>",
                            error.to_string()
                        )
                    );
                    println!("Funcione luego de error");
                }
            }
            db_file_model.end_of_process();
            loading.end();
            sender.send(1 as i64).unwrap();
        });

    }


    pub fn print_total_files(&mut self) {
        let mut loading = Loading::new();
        loading.start();
        loading.info("Iniciando conexión con FTP");

        let mut ftp_client= self.ftp_client.lock().unwrap();
        ftp_client.binary().unwrap();
        loading.success("Conexión exitosa con FTP");

        let files = ftp_client.list_names(&self.ftp_atributes.directory_guias_cpm).unwrap();
        loading.info(format!("Total archivos en FTP: {}", files.len()));
        loading.end();
        ftp_client.logout().unwrap();
    }


}

