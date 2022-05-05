use serde::{Serialize, Deserialize};
use sqlx::{Error, Pool, MySql, FromRow};


#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct NewsLists {
	pub id: u32,
	pub name: String,
	pub age: u32,
}


pub async fn sql_news_lists(pool: &Pool<MySql>) ->  Result<Vec<NewsLists>, Error> {
	let records:Vec<NewsLists> = sqlx::query_as::<_, NewsLists>(
		r#"
	SELECT * FROM news
	"#,
).fetch_all(pool).await?;
	return Ok(records);
}