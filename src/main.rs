use linkshortener::{establish_connection};
use linkshortener::schema::*;
use linkshortener::models::*;

extern crate diesel;

fn main() {
    use linkshortener::schema::shortlinks::dsl::*;
    use crate::diesel::QueryDsl;
    use crate::diesel::RunQueryDsl;

    println!("Starting...");

    let connection = establish_connection();

    let results = shortlinks::limit(shortlinks, 5)
        .load::<ShortLink>(&connection)
        .expect("Error loading shortened links");

    println!("Displaying {} shortened links", results.len());
    for shortlink in results {
        println!("{}", shortlink.id);
        println!("----------\n");
        println!("{} \n\n\n", shortlink.link);
    }

}
