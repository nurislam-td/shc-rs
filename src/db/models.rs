use diesel::prelude::{Insertable, Queryable};

use crate::db::schema::connection;


#[derive(Queryable, Debug)]
pub struct SshConnection {
    pub id: i32,
    pub name: String,
    pub password: Vec<u8>,
    pub host: String,
    pub port: i32,
    pub username: String,
}


#[derive(Insertable)]
#[diesel(table_name = connection)]
pub struct NewConnection<'a> {
    pub name: &'a str,
    pub password: &'a Vec<u8>,
    pub host: &'a str,
    pub port: i32,
    pub username: &'a str,
}
