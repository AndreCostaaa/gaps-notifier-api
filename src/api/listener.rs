use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use super::state::ApiState;
use crate::db::db::Database;
use crate::models::listener::Listener;

pub fn create_listener(State(db): State<ApiState>) -> impl IntoResponse {
    let listener = Listener::new_with_random_uuid();

    Json(listener)
}

pub async fn get_listener(
    State(apiState): State<ApiState>,
    listener_id: Path<u64>,
) -> impl IntoResponse {
    let listener: Option<Listener> = apiState.redis_db.fetch(listener_id);

    match listener {
        Some(listener) => Json(listener),
        None => StatusCode::NOT_FOUND,
    }
}
pub fn routes() -> axum::routing::Router {
    axum::Router::new()
        .route("/listener", axum::routing::post(create_listener))
        .route("/listener/:listener_id", axum::routing::get(get_listener))
}
