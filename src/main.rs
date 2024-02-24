use axum::{routing::get, Router};
pub mod api;
pub mod db;
pub mod logic;
pub mod models;
use api::course_listener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/api", api::listener::routes())
        .nest("/api", course_listener::routes())
        .nest("/api", api::grade::routes());
    // axum::Server::bind(&"
}
