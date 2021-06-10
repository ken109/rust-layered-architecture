use std::env;

use diesel::mysql::MysqlConnection;
use diesel::{Connection};

pub mod task;


pub fn connect() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").
        expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url).
        expect(&format!("Error connecting to {}", database_url))
}
