use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

const URI: &str = "mysql://demo:demo@localhost:3306/demo";

pub async fn mysql_connect() -> Pool<MySql> {
	let pool = MySqlPoolOptions:: new()
	.max_connections(8)
	.connect(&URI).await;
	pool.unwrap()
}