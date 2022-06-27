use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::{Utc};


#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct NewsLists {
	pub id: u32,
	pub name: String,
	// pub age: u32,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Lists {
	pub id: i32,
	pub href: String,
	pub img_url: String,
	pub load_img: String,
	pub title: String,
	pub country: String,
	pub source: String,
	pub description: String,
	pub status: i8,
	pub create_time: chrono::DateTime<Utc>,
	pub menu: String,
	pub href_hash: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Details {
	pub id: i32,
	pub news_id: i32,
	pub figure: String,
	pub p_list: String,
	pub alt: String,
	pub src: String,
	pub local_src: String,
	pub create_time: chrono::DateTime<Utc>,
	pub content: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Minidetails {
	pub id: i32,
	pub news_id: i32,
	pub src: String,
	pub create_time: chrono::DateTime<Utc>,
	pub content: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct LasterLists {
	pub id: i32,
	pub href: String,
	pub img_url: String,
	pub load_img: String,
	pub title: String,
	pub country: String,
	pub source: String,
	pub description: String,
	pub status: i8,
	pub create_time: chrono::DateTime<Utc>,
	pub menu: String,
	pub href_hash: String,

	// pub figure: String,
	// pub p_list: String,
	// pub alt: String,
	pub src: String,
	pub local_src: String,
	pub content: String,

}

