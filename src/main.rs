use std::{net::SocketAddr, sync::Arc};
use axum::{Router, Extension, routing::get, http::{HeaderValue, Method}};
use panda::{AppState, utils::{model}};
use tower_http::{trace::TraceLayer, cors::CorsLayer};
mod config;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 6000));
    let pool = config::db::mysql_connect().await;
    // let mut con = config::db::redis_connect().await;
    let con = model::CacheService::new().unwrap();
    let app = Router::new()
        .route("/", get(||async{ "Welcome to use serv_rs"}))
        .nest("/api/v1", config::routes::app_api())
        .layer(Extension(Arc::new(AppState { pool, con })))
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
            .allow_origin("http://localhost:6000".parse::<HeaderValue>().unwrap())
            .allow_methods(vec![Method::GET])
        );

    println!("Start server IP: {}", addr);
    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();    
}