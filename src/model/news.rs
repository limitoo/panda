use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct NewsLists {
	pub id: u32,
	pub name: String,
	pub age: u32,
}

