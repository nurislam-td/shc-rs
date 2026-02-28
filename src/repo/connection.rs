use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::db::{
    models::{NewConnection, SshConnection},
    schema::connection,
};

pub fn create(
    conn: &mut SqliteConnection,
    name: &str,
    password: &Vec<u8>,
    host: &str,
    username: &str,
) {
    let new_conn = NewConnection {
        name,
        password,
        host,
        port: 22,
        username,
    };

    diesel::insert_into(connection::table)
        .values(&new_conn)
        .execute(conn)
        .expect("Insert error");
}

pub fn list_connections(conn: &mut SqliteConnection) -> Vec<SshConnection> {
    connection::table
        .load::<SshConnection>(conn)
        .expect("Select error")
}

pub fn update_name(conn: &mut SqliteConnection, conn_id: i32, new_name: &str) {
    diesel::update(connection::table.find(conn_id))
        .set(connection::name.eq(new_name))
        .execute(conn)
        .expect("Update name error");
}

pub fn delete_connection(conn: &mut SqliteConnection, conn_id: i32) {
    diesel::delete(connection::table.find(conn_id))
        .execute(conn)
        .expect("Delete error");
}
