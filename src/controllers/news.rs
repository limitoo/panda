use sqlx::{Error, Pool, MySql};

use crate::model::news::NewsLists;


pub async fn sql_news_lists(pool: &Pool<MySql>) ->  Result<Vec<NewsLists>, Error> {
	let records:Vec<NewsLists> = sqlx::query_as::<_, NewsLists>(
		r#"
			SELECT * FROM news
		"#,
		).fetch_all(pool).await?;
	return Ok(records);
}

pub async fn sql_news_id(pool: &Pool<MySql>, id: i32) ->  Result<NewsLists, Error> {
	sqlx::query_as::<_, NewsLists>(
		r#"
			SELECT * FROM news
			WHERE id = ?
		"#,
		)
		.bind(id)
		.fetch_one(pool).await
}


