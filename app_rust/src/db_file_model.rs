use std::sync::{Arc, Mutex};
use chrono::Local;
use postgres::Client;

pub struct DbFileModel {
    execution_id: i32,
    file_id: Option<i64>,
    connection_db: Arc<Mutex<Client>>,
}
impl DbFileModel {

    pub fn new(execution_id: i32, connection_db: Arc<Mutex<Client>>) -> Self {
        DbFileModel{
            execution_id,
            file_id: None,
            connection_db
        }
    }

    pub fn start_of_process(&mut self, name_file: &String) {
        let now= Local::now().naive_local();
        let query_result= self.connection_db.lock().unwrap().query_one(
            "INSERT INTO files (start_at, name_file, execution_id) VALUES ($1, $2, $3) returning id",
            &[ &now, &name_file, &self.execution_id],
        ).unwrap();

        let id: i64 = query_result.get(0);
        self.file_id= Some(id);
    }

    pub fn end_of_process(&mut self) {
        let now= Local::now().naive_local();
        self.connection_db.lock().unwrap().execute(
            "UPDATE files SET end_at = $1 WHERE id = $2",
            &[&now, &self.file_id],
        ).unwrap();

    }

}