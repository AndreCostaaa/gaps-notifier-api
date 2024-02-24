use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    routing::{get, post},
    Router,
};
pub mod api;
pub mod db;
pub mod discord;
pub mod logic;
pub mod models;

use std::{env, time::Duration};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let db = db::redis::RedisDb::new(&env::var("REDIS_URL").expect("REDIS_URL not set")).await;

    let app = Router::new()
        .route(
            "/api/listener",
            get(api::listener::get_listener).post(api::listener::post_listener),
        )
        .route(
            "/api/course_listener",
            post(api::course_listener::register_course_listener),
        )
        .route("/api/grade", post(api::grade::create_grade))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(api::state::ApiState { redis_db: db });

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(tcp_listener, app).await.unwrap();
}
