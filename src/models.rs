#[derive(Queryable)]
pub struct Link {
    pub id: i32,
    pub short: String,
    pub hyperlink: String,
	pub enabled: bool,
}