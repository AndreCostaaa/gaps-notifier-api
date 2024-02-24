use axum::{extract::State, response::IntoResponse, Json};

use crate::models::course_listener::CourseListener;

use super::state::ApiState;

pub fn register_course_listener(
    State(mut apiState): State<ApiState>,
    body: Json<CourseListener>,
) -> impl IntoResponse {
}
pub fn routes() -> axum::routing::Router {
    axum::Router::new().route(
        "/course_listener",
        axum::routing::post(register_course_listener),
    )
}
