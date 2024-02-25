use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use super::authorization::Token;
use super::state::ApiState;
use crate::logic;

pub async fn create_user(token: Token, State(mut api_state): State<ApiState>) -> impl IntoResponse {
    if !token.is_admin {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    let user = logic::user::create_user(&mut api_state.redis_db).await;

    match user {
        Some(user) => (StatusCode::OK, Json(user)).into_response(),
        None => StatusCode::CONFLICT.into_response(),
    }
}

pub async fn get_user(
    State(mut api_state): State<ApiState>,
    token: Token,
    user_id: Path<u128>,
) -> impl IntoResponse {
    if !token.is_admin {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let user = logic::user::get_user(&mut api_state.redis_db, user_id.0).await;

    match user {
        Some(user) => (StatusCode::OK, Json(user)).into_response(),
        None => StatusCode::NOT_FOUND.into_response().into_response(),
    }
}
