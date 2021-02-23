use std::iter;
use std::mem;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

use super::schema::links;



#[derive(Queryable)]
pub struct Link {
    pub id: i32,
    pub shorttext: String,
    pub hyperlink: String,
	pub enabled: bool,
}

#[derive(Insertable)]
#[table_name="links"]
pub struct NewLink {
    pub shorttext: String,
    pub hyperlink: String,
}

impl NewLink {

    pub fn new(link: &str) -> NewLink {
        let shorttext: String = NewLink::generate_short();
        let hyperlink: String = link.trim().to_string();

        mem::drop(link);

        assert!(!hyperlink.is_empty());

        NewLink { shorttext, hyperlink }
    }
    
    fn generate_short() -> String {
        // FROM: https://docs.rs/rand/0.8.3/rand/distributions/struct.Alphanumeric.html
        let mut rng = thread_rng();
        let chars: String = iter::repeat(())
                .map(|()| rng.sample(Alphanumeric))
                .map(char::from)
                .take(7)
                .collect();
        return chars;
    }
}