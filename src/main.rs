use std::{io::stdin, mem};

use linkshortener::{create_link, establish_connection};

extern crate diesel;


#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: i8,
}


impl Person {
    fn new(first: &str, last: &str, age: i8) -> Person {
        let first_name: String = first.to_string();
        let last_name: String = last.to_string();

        mem::drop(first);
        mem::drop(last);


        assert!(age >= 0);
        assert!(first_name != "");
        assert!(last_name != "");

        Person { first_name, last_name, age }
    }

    fn details(&self) -> String {
        return format!("First Name: {}, Last Name {}, Age: {}", self.first_name, self.last_name, self.age);
    }
}

fn main() {

    /*
    let p = Person::new("Dalen", "Ward", 22);

    let mut people = Vec::<Person>::new();
    people.push(Person::new("Amelia", "Ward", 19));
    people.push(p);


    println!("{:?}", people);

    for p in people {
        println!("{}", p.details());
    }
    */

    let connection = establish_connection();

    println!("What would you like your link to be?");
    let mut hyperlink = String::new();
    stdin().read_line(&mut hyperlink).unwrap();

    let link = create_link(&connection, &hyperlink.to_string());
    println!("\nSaved link {} with id {}. The shortlink is {}", hyperlink, link.id, link.shorttext);
}
