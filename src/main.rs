use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing::get, Router};
pub mod api;
pub mod db;
pub mod logic;
pub mod models;
use api::{course_listener, grade, listener, state::ApiState};
use std::{env, sync::Arc, time::Duration};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let db = db::redis::RedisDb::new(&env::var("REDIS_URL").expect("REDIS_URL not set")).await;

    let appState = ApiState { redis_db: db };

    let app = Router::new()
        .nest("/api", api::listener::routes())
        .nest("/api", course_listener::routes())
        .nest("/api", api::grade::routes())
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
        .with_state(appState);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
