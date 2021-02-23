#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use models::{Link, NewLink};

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_link<'a>(conn: &PgConnection, link: &str) -> Link {
    use schema::links;

    let new_link = NewLink::new(link);

    diesel::insert_into(links::table)
        .values(&new_link)
        .get_result(conn)
        .expect("Error saving new post")
}