struct Person {
    first_name: String,
    last_name: String,
    age: i8,
}

impl Person {
    fn new(first: &str, last: &str, age: i8) -> Person {
        let first_name: String = first.to_string();
        let last_name: String = last.to_string();


        assert!(age >= 0);
        assert!(first_name != "");
        assert!(last_name != "");

        Person { first_name, last_name, age }
    }

    fn details(&self) -> String {
        format!("First Name: {}, Last Name {}, Age: {}", self.first_name, self.last_name, self.age)
    }
}

fn main() {
    let p = Person::new("Dalen", "Ward", 22);
    println!("{}", p.details());
}
