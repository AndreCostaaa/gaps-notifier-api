use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::models::course_listener::CourseListener;

use super::state::ApiState;

pub async fn register_course_listener(
    State(mut api_state): State<ApiState>,
    Json(body): Json<CourseListener>,
) -> impl IntoResponse {
    let course_listener =
        crate::logic::course_listener::register(&mut api_state.redis_db, &body).await;
    match course_listener {
        true => (StatusCode::OK, Json(body)).into_response(),
        false => StatusCode::CONFLICT.into_response(),
    }
}
