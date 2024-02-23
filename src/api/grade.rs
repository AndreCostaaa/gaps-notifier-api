use crate::{
    db::db::Database,
    logic::utils::current_school_year,
    models::{course_listener::CourseListener, grade::Grade, hashing::calculate_hash},
};

use super::state::ApiState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct GradeCreateArgs {
    course: String,
    name: String,
    class: String,
    class_average: f64,
}

pub fn create_grade(
    State(mut apiState): State<ApiState>,
    body: Json<GradeCreateArgs>,
) -> impl IntoResponse {
    let grade = Grade::new(
        body.course.clone(),
        body.class.clone(),
        current_school_year(),
        body.name.clone(),
        body.class_average,
    );

    let grade_hash = calculate_hash(&grade);
    let grade_fetch: Option<Grade> = apiState.redis_db.fetch(grade_hash)
    if let Some(_) = grade_fetch {
        return StatusCode::CONFLICT;
    }

    match apiState.redis_db.save(&grade) {
        true => StatusCode::CREATED,
        false => StatusCode::INTERNAL_SERVER_ERROR,
    }
    //TODO notify listeners
    //let course_listener_key = CourseListener::compute_key(&grade.class, &grade.course, grade.year);
    // get course listeners using course_listener_key
    // for each listener, send a notification using webhook_url
}

pub fn routes() -> axum::routing::Router {
    axum::Router::new().route("/grade", axum::routing::post(create_grade))
}
