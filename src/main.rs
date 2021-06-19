use std::io::stdin;
use linkshortener::{establish_connection};

extern crate diesel;

fn main() {

    println!("Starting...");

    let connection = establish_connection();

}
