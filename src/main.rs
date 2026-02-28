use diesel::{sqlite::SqliteConnection, Connection};

mod db;
mod repo;
mod service;

fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("shc.db").expect("Database connection failed")
}

fn main() {
    let mut conn = establish_connection();
}
