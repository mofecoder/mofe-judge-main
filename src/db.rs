use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let db_name = env::var("DB_NAME").expect("db_name must be set");
    let db_user = env::var("DB_USER").expect("db_user must be set");
    let db_pass = env::var("DB_PASS").expect("db_pass must be set");
    let db_host = env::var("DB_HOST").expect("db_host must be set");
    let db_port = env::var("DB_PORT").expect("db_port must be set");

    let db_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db_user, db_pass, db_host, db_port, db_name
    );

    MysqlConnection::establish(&db_url).expect("error connect to db")
}
