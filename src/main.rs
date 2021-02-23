use std::io::stdin;
use linkshortener::{create_link, establish_connection};

extern crate diesel;

fn main() {

    let connection = establish_connection();

    println!("What would you like your link to be?");
    let mut hyperlink = String::new();
    stdin().read_line(&mut hyperlink).unwrap();

    let link = create_link(&connection, &hyperlink.to_string());
    println!("\nSaved link {} with id {}. The shortlink is {}", hyperlink, link.id, link.shorttext);
}
