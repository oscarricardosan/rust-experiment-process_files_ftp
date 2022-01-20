use std::{fmt, thread};
use std::sync::{Arc, mpsc, Mutex};
use ftp_client::prelude::Client;
use crate::config_app::FtpAttributes;

type Job = Box<dyn FnOnce(Arc<Mutex<Client>>) + Send + 'static>;
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
    thread: Option<thread::JoinHandle<()>>
}

impl Worker{
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>, ftp_attrs: &FtpAttributes)-> Worker {

        let mut ftp_client= Client::connect(
            &ftp_attrs.ftp_url,
            &ftp_attrs.ftp_user,
            &ftp_attrs.ftp_password
        ).unwrap();
        ftp_client.binary().unwrap();

        let ftp_client= Arc::new(Mutex::new(ftp_client));
        println!("Conección con FTP para worker {} establecida exitosamente", id);
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    // println!("Worker {} got a job; executing.", id);
                    job(ftp_client.clone());
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

    pub fn new(size: usize,ftp_attrs: FtpAttributes) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Err(PoolCreationError);
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(
                Worker::new(
                    id,
                    Arc::clone(&receiver),
                    &ftp_attrs
                )
            )
        }
        Ok(ThreadPool{
            workers, sender
        })
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce(Arc<Mutex<Client>>) + Send + 'static,
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