use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
mod db;

use crate::db::models::{SshConnection, NewConnection};
use crate::db::schema::connection;
use diesel::sqlite::SqliteConnection;

fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("shc.db").expect("Ошибка подключения к базе данных")
}

// CREATE
fn create_connection(conn: &mut SqliteConnection, name: &str, password: &str, host: &str) {
    let new_conn = NewConnection {
        name,
        password,
        host,
    };

    diesel::insert_into(connection::table)
        .values(&new_conn)
        .execute(conn)
        .expect("Ошибка вставки");
}

// READ
fn list_connections(conn: &mut SqliteConnection) -> Vec<SshConnection> {
    connection::table
        .load::<SshConnection>(conn)
        .expect("Ошибка выборки")
}

// UPDATE
fn update_name(conn: &mut SqliteConnection, conn_id: i32, new_name: &str) {
    diesel::update(connection::table.find(conn_id))
        .set(connection::name.eq(new_name))
        .execute(conn)
        .expect("Ошибка обновления");
}

// DELETE
fn delete_connection(conn: &mut SqliteConnection, conn_id: i32) {
    diesel::delete(connection::table.find(conn_id))
        .execute(conn)
        .expect("Ошибка удаления");
}

fn main() {
    let mut conn = establish_connection();

    // CREATE
    create_connection(&mut conn, "server1", "secret", "127.0.0.1");

    // READ
    let all = list_connections(&mut conn);
    println!("Все подключения: {:?}", all);

    // UPDATE
    if let Some(first) = all.first() {
        update_name(&mut conn, first.id, "updated_server");
    }
    let all = list_connections(&mut conn);
    println!("Все подключения: {:?}", all);

    // DELETE
    if let Some(first) = all.first() {
        delete_connection(&mut conn, first.id);
    }

    let all = list_connections(&mut conn);
    println!("Все подключения: {:?}", all);


}
