extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
use diesel::{prelude::*, sqlite::SqliteConnection};
embed_migrations!("migrations/");

fn establish_connection(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(database_url).unwrap()
}

fn main() {
    let connection = establish_connection("test.db");
    crate::embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).unwrap();
}
