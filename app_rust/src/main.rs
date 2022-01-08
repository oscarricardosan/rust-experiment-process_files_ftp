mod config_app;
mod ftp;
mod thread_pool;

use clap::{App, Arg, SubCommand};
use crate::config_app::ConfigApp;
use crate::ftp::Ftp;


fn main() {
    let matches = App::new("FtpReader")
        .version("1.0")
        .author("Oscar Sánchez. <oscar.sanchez@savne.net>")
        .about("Aplicación para la conexión a FTP y obtención de archivos con un pool de 20 conexiones")
        .subcommand(
            SubCommand::with_name("setup")
                .about("Configurar aplicación")
                .version("1.0")
                .author("Oscar Sánchez")
                .arg(
                    Arg::with_name("show")
                        .help("mostrar configuración de aplicación")
                        .long("show")
                        .short("s")
                ),
        )
        .subcommand(
            SubCommand::with_name("start")
                .about("Iniciar obtención y procesamiento de imágenes")
                .version("1.0")
                .author("Oscar Sánchez")
        )
        .get_matches();


    let mut config_app = ConfigApp::new();

    if !config_app.is_configured(){
        config_app.require_config_data();
    }

    // You can also match on a subcommand's name
    match matches.subcommand_name() {
        Some("setup") => {
            let math_subcommand= matches.subcommand_matches("setup").unwrap();
            if math_subcommand.is_present("show") {
                config_app.show_config();
            } else {
                config_app.require_config_data();
            }
        },
        Some("start") => {
            let mut ftp= Ftp::new(config_app);
            ftp.start_image_processing();
            // dbg!(ftp);
            // ftp::start_image_processing(config_app.clone())
        },
        None => println!("Subcomando a ejecutar no especificado, para mas información especifique la bandera --help."),
        _ => println!("Subcomando especificado no es reconocido, para mas información especifique la bandera --help."),
    }


}