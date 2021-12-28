use std::io;
use std::io::Write;
use std::process::exit;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigApp {
    pub ftp_url: String,
    pub ftp_user: String,
    pub ftp_password: String,

    pub directory_guias_cpm: String,

    is_it_configured: bool,
    version: f32,
}
impl Default for ConfigApp {
    fn default() -> Self {
        Self {
            ftp_url: String::new(),
            ftp_user: String::new(),
            ftp_password: String::new(),

            directory_guias_cpm: String::new(),

            is_it_configured: false,
            version: 1.0,
        }
    }
}

impl ConfigApp {

    pub fn new()-> Self {
        let config_app: ConfigApp = confy::load("app-rust-ftp").unwrap_or_default();
        config_app
    }

    pub fn is_configured(&self)-> bool{
        self.is_it_configured && self.version == 1.0
    }

    pub fn require_config_data(&mut self){

        println!("************************************************");
        println!("**         DATOS DE ACCESO A FTP ECLIPSE      **");
        println!("************************************************\n\n");
        println!("Para poder ejecutar el programa correctamente es necesario que ingreses los siguientes datos:\n");

        print!(" * Url: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut self.ftp_url)
            .expect("Fallo al leer línea");

        print!(" * Usuario: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut self.ftp_user)
            .expect("Fallo al leer línea");

        print!(" * Contraseña: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut self.ftp_password)
            .expect("Fallo al leer línea");

        print!(" * Ruta de directorio guías CPM: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut self.directory_guias_cpm)
            .expect("Fallo al leer línea");

        self.ftp_url= self.ftp_url.trim().to_string();
        self.ftp_user= self.ftp_user.trim().to_string();
        self.ftp_password= self.ftp_password.trim().to_string();
        self.directory_guias_cpm= self.directory_guias_cpm.trim().to_string();
        self.is_it_configured= true;
        confy::store("app-rust-ftp", self).unwrap();

        println!("************");
        print!(" Configuración exitosa, desea correr imagenes? (Si/No): ");
        io::stdout().flush().unwrap();

        let mut run_images: String= String::new();
        io::stdin()
            .read_line(&mut run_images)
            .expect("Fallo al leer línea");

        run_images= run_images.trim().to_lowercase();

        println!("{}", run_images);
        if run_images != "si" {
            exit(exitcode::OK)
        };
    }
}