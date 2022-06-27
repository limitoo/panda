use std::sync::Arc;
use axum::{Extension, response::IntoResponse, extract};
use crate::{AppState, controllers::news, RespVO};

pub async fn get_news(
	Extension(state): Extension<Arc<AppState>>
) -> impl IntoResponse {
	match news::sql_news_lists(&state.pool).await {
		Ok(recode) => {
			return RespVO::from(&recode).resp_json();
		}
		Err(_err) => {
			return RespVO::from(&"error".to_string()).resp_json();
		}
	}
}


pub async fn get_details(
	extract::Path(id): extract::Path<i32>,
  Extension(state): Extension<Arc<AppState>>
) -> impl IntoResponse {
	match news::sql_news_id(&state.pool, id).await {
		Ok(recode) => {
			return RespVO::from(&recode).resp_json();
		}
		Err(_err) => {
			return RespVO::from(&"error".to_string()).resp_json();
		}
	}
}

pub async fn get_laster(
	Extension(state): Extension<Arc<AppState>>
) -> impl IntoResponse {
	match news::sql_laster_100(&state.pool, 50).await {
		Ok(recode) => {
			return RespVO::from(&recode).resp_json();
		}
		Err(_err) => {
			return RespVO::from_err(&"error".to_string()).resp_json();
		}
	}
}

pub async fn get_hotnews(
	Extension(state): Extension<Arc<AppState>>
) -> impl IntoResponse {
	match news::sql_hotnews(&state.pool, 5).await {
		Ok(recode) => {
			return RespVO::from(&recode).resp_json();
		}
		Err(_err) => {
			return RespVO::from_err(&"error".to_string()).resp_json();
		}
	}
}
