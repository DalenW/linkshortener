use std::iter;
use std::mem;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;



#[derive(Queryable)]
pub struct Link {
    pub id: i32,
    pub short: String,
    pub hyperlink: String,
	pub enabled: bool,
}

impl Link {

    fn new(link: &str) -> Link {
        let id = 0;
        let short: String = Link::generate_short();
        let enabled: bool = true;
        let hyperlink: String = link.to_string();

        mem::drop(link);

        assert!(hyperlink != "");

        Link { id, short, hyperlink, enabled }
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