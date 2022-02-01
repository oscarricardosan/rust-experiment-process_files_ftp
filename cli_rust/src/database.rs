use postgres::{Client, NoTls};

//https://docs.rs/postgres/0.16.0-rc.2/postgres/types/trait.FromSql.html
pub fn get_connection_postgres()->Client {
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