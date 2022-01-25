use chrono::Local;
use postgres::Client;
use crate::database;

pub struct DbExecutionModel {
    execution_id: Option<i32>,
    connection_db: Client,
}
impl DbExecutionModel {

    pub fn new() -> Self {
        DbExecutionModel{
            execution_id: None,
            connection_db: database::get_connection_postgres()
        }
    }

    pub fn start_of_process(&mut self, total_files:i32) {
        let now= Local::now().naive_local();
        let query_result= self.connection_db.query_one(
            "INSERT INTO executions (start_at, total_files) VALUES ($1, $2) returning id",
            &[ &now, &total_files],
        ).unwrap();

        let id: i32 = query_result.get(0);
        self.execution_id= Some(id);
    }

    pub fn end_of_process(&mut self, files_processed_successfully:i32) {
        let now= Local::now().naive_local();
        self.connection_db.execute(
            "UPDATE executions SET end_at = $1, files_processed_successfully = $2 WHERE id = $3",
            &[&now, &files_processed_successfully, &self.execution_id],
        ).unwrap();

    }

    pub fn get_execution_id(&mut self)->i32 {
        self.execution_id.unwrap().clone()
    }

}