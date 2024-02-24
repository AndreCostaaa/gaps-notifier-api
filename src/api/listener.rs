use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use super::state::ApiState;
use crate::logic;

pub async fn post_listener(State(mut api_state): State<ApiState>) -> impl IntoResponse {
    let listener = logic::listener::create_listener(&mut api_state.redis_db).await;
    match listener {
        Some(listener) => (StatusCode::OK, Json(listener)).into_response(),
        None => StatusCode::CONFLICT.into_response(),
    }
}

pub async fn get_listener(
    State(mut api_state): State<ApiState>,
    Path(listener_id): Path<u128>,
) -> impl IntoResponse {
    let listener = logic::listener::get_listener(&mut api_state.redis_db, listener_id).await;

    match listener {
        Some(listener) => (StatusCode::OK, Json(listener)).into_response(),
        None => StatusCode::NOT_FOUND.into_response().into_response(),
    }
}
