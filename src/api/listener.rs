use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use super::authorization::Token;
use super::state::ApiState;
use crate::logic;

pub async fn post_listener(
    token: Token,
    State(mut api_state): State<ApiState>,
) -> impl IntoResponse {
    if !token.is_admin {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    let listener = logic::listener::create_listener(&mut api_state.redis_db).await;

    match listener {
        Some(listener) => (StatusCode::OK, Json(listener)).into_response(),
        None => StatusCode::CONFLICT.into_response(),
    }
}

pub async fn get_listener(
    State(mut api_state): State<ApiState>,
    token: Token,
    listener_id: Path<u128>,
) -> impl IntoResponse {
    if !token.is_admin {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let listener = logic::listener::get_listener(&mut api_state.redis_db, listener_id.0).await;

    match listener {
        Some(listener) => (StatusCode::OK, Json(listener)).into_response(),
        None => StatusCode::NOT_FOUND.into_response().into_response(),
    }
}
