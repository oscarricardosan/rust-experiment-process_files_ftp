use std::{fs, thread};
use std::process::exit;
use std::sync::mpsc;
use chrono::Local;
use loading::Loading;
use crate::ConfigApp;

use std::fmt;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use ftp_client::client::Client;

type Job = Box<dyn FnOnce() + Send + 'static>;
pub enum Message {
    NewJob(Job),
    Terminate,
}

#[derive(Debug, Clone)]
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "El tamaño de un pool debe ser mayor a cero")
    }
}

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

pub struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker{
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>)-> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    // println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}

impl ThreadPool {

    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Err(PoolCreationError);
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }
        Ok(ThreadPool{
            workers, sender
        })
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


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

    let pool = ThreadPool::new(40).unwrap_or_else(
        |err|{
            println!("Error al generar pool de conexiones: {}", err);
            exit(1)
        }
    );

    let (tx, rx) = mpsc::channel();

    let files= get_files();
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
        sender.send(1 as i64).unwrap();
    });

}