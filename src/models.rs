use diesel::{Queryable};
use chrono::{ DateTime, Utc };

//use super::schema::*;



#[derive(Queryable)]
pub struct ShortLink {
    pub id: String,
    pub link: String,
    pub created_at: DateTime<Utc>,
	pub enabled: bool,
    pub hits: i64
}
