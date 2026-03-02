mod db;
mod repo;
mod service;
mod terminal;

use color_eyre::eyre::Result;
use diesel::{sqlite::SqliteConnection, Connection};

fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("shc.db").expect("Database connection failed")
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut conn = establish_connection();
    ratatui::run(terminal::event_loop::run)?;
    Ok(())
}
