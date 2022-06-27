use sqlx::{Error, Pool, MySql};

use crate::model::news::{NewsLists, Minidetails, LasterLists};


pub async fn sql_news_lists(pool: &Pool<MySql>) ->  Result<Vec<NewsLists>, Error> {
	let records:Vec<NewsLists> = sqlx::query_as::<_, NewsLists>(
		r#"
			SELECT id, name FROM news order by id desc LIMIT 3;
		"#,
		).fetch_all(pool).await?;
	return Ok(records);
}

pub async fn sql_news_id(pool: &Pool<MySql>, id: i32) ->  Result<Minidetails, Error> {
	sqlx::query_as::<_, Minidetails>(
		r#"
			SELECT id, news_id, src, create_time, content  FROM details WHERE id = ?
		"#,
		)
		.bind(id)
		.fetch_one(pool).await
}

// select ny.id, nyd.content from news ny, details nyd where ny.id = nyd.news_id order by ny.create_time desc limit 10;
pub async fn sql_laster_100(pool: &Pool<MySql>, num: i32) ->  Result<Vec<LasterLists>, Error> {
	let records:Vec<LasterLists> = sqlx::query_as::<_, LasterLists>(
		r#"
		select ny.href,
		ny.img_url,
		ny.load_img,
		ny.title,
		ny.country,
		ny.source,
		ny.description,
		ny.status,
		ny.create_time,
		ny.menu,
		ny.href_hash,
		nyd.id,
		nyd.local_src,
		nyd.src,
		nyd.content from news ny, details nyd where ny.id = nyd.news_id order by ny.create_time desc limit ?;
		"#,
	).bind(num)
	.fetch_all(pool).await?;
	return Ok(records);
}

pub async fn sql_hotnews(pool: &Pool<MySql>, num: i32) ->  Result<Vec<LasterLists>, Error> {
	let records:Vec<LasterLists> = sqlx::query_as::<_, LasterLists>(
		r#"
		select ny.id, ny.href,
		ny.img_url,
		ny.load_img,
		ny.title,
		ny.country,
		ny.source,
		ny.description,
		ny.status,
		ny.create_time,
		ny.menu,
		ny.href_hash,
		nyd.local_src,
		nyd.src,
		nyd.content from news ny, details nyd where ny.id = nyd.news_id order by ny.create_time desc limit ?;
		"#,
	).bind(num)
	.fetch_all(pool).await?;
	return Ok(records);
}

