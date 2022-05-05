
use axum::{Router};
use axum::routing::{get};
use panda::app::news::{get_news, get_details};

pub fn app_api() -> Router {
	Router::new()
	.route("/news", get(get_news))
	.route("/:id", get(get_details))
}