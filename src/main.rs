mod db;
mod repo;
mod service;
mod terminal;

use color_eyre::eyre::Result;
use diesel::{sqlite::SqliteConnection, Connection};

use terminal::event_loop;

fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("shc.db").expect("Database connection failed")
}

fn main() -> Result<()> {
    let mut conn = establish_connection();
    let mut terminal = ratatui::init();
    event_loop::run(&mut terminal)?;

    Ok(())
}
