use super::state::ApiState;
use crate::logic::grade;
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct GradeCreateArgs {
    course: String,
    name: String,
    class: String,
    class_average: f64,
}

pub async fn create_grade(
    State(mut apiState): State<ApiState>,
    Json(body): Json<GradeCreateArgs>,
) -> impl IntoResponse {
    match grade::register_grade(
        &mut apiState.redis_db,
        body.course,
        body.name,
        body.class,
        body.class_average,
    ) {
        true => (StatusCode::CREATED, "Grade created"),
        false => (StatusCode::CONFLICT, "Grade already exists"),
    }
}

pub fn routes() -> axum::routing::Router {
    axum::Router::new().route("/grade", post(create_grade))
}
