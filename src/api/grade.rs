use super::{authorization::Token, state::ApiState};
use crate::logic;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GradeCreateArgs {
    course: String,
    name: String,
    class: String,
    class_average: f64,
}

pub async fn create_grade(
    State(mut api_state): State<ApiState>,
    _token: Token,
    body: Json<GradeCreateArgs>,
) -> impl IntoResponse {
    let body = body.0;
    match logic::grade::register_grade(
        &mut api_state.redis_db,
        body.course,
        body.name,
        body.class,
        body.class_average,
    )
    .await
    {
        true => (StatusCode::CREATED, "Grade created").into_response(),
        false => (StatusCode::CONFLICT, "Grade already exists").into_response(),
    }
}
