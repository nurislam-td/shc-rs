use diesel::prelude::{Insertable, Queryable};

use crate::db::schema::connection;

// Структура для выборки (SELECT)
#[derive(Queryable, Debug)]
pub struct SshConnection {
    pub id: i32,
    pub name: String,
    pub password: Vec<u8>,
    pub host: String,
    pub port: i32,
}

// Структура для вставки (INSERT)
#[derive(Insertable)]
#[diesel(table_name = connection)]
pub struct NewConnection<'a> {
    pub name: &'a str,
    pub password: &'a Vec<u8>,
    pub host: &'a str,
    pub port: i32,
}
