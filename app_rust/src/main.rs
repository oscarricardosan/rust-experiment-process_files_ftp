mod config_app;
mod ftp;
mod thread_pool;

use chrono::DateTime;
use clap::{App, Arg, SubCommand};
use loading::Loading;
use postgres::{Client, NoTls};
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
        .subcommand(
            SubCommand::with_name("total-files")
                .about("Cuenta los archivos disponibles en la carpeta ftp")
                .version("1.0")
                .author("Oscar Sánchez")
        )
        .subcommand(
            SubCommand::with_name("show-setup")
                .about("Muestra la configuración de la aplicación")
                .version("1.0")
                .author("Oscar Sánchez")
        )
        .get_matches();

    let mut client= get_connection_postgres();

    let name = DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z");
    let data:i8 = 15;
    client.execute(
        "INSERT INTO executions (start_at, total_files) VALUES ($1, $2)",
        &[&name, &data],
    ).unwrap();

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

            let mut ftp= Ftp::new(config_app.get_ftp_attributes());
            ftp.start_image_processing();
        },
        Some("total-files") => {
            let mut ftp= Ftp::new(config_app.get_ftp_attributes());
            ftp.print_total_files();
        },
        Some("show-setup") => {
            config_app.show_config();
        },
        None | _ => {
            print_message_no_command();
        }
    }

}

fn print_message_no_command() {
    print!("\n\n");
    let mut loading = Loading::new();
    loading.start();
    loading.warn("Subcomando a ejecutar no valido, para más información sobre el uso de esta aplicación ejecute el comando --help\n\n");
    loading.end();
}

fn get_connection_postgres()->Client {
    let user= "savne";
    let password= "secret";
    let host= "app-rust-ftp-postgres:5432";
    let db= "ftp";
    let mut client= Client::connect(
    format!("postgresql://{}:{}@{}/{}", user, password, host, db).as_str()
    , NoTls
    ).unwrap();
    client
}