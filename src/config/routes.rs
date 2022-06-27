
use axum::{Router};
use axum::routing::{get};
use panda::app::news::{get_news, get_details, get_laster, get_hotnews};

pub fn app_api() -> Router {
	Router::new()
	.route("/news", get(get_news))
	.route("/:id", get(get_details))
	.route("/laster", get(get_laster))
	.route("/hot", get(get_hotnews))
}