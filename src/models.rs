use chrono::prelude::DateTime;
use diesel::types::Timestamp;

use super::schema::*;



#[derive(Queryable)]
pub struct ShortLink {
    pub id: String,
    pub link: String,
    pub createdAt: Timestamp,
	pub enabled: bool,
    pub hits: i64
}
