use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{logic::time, models::course_listener::CourseListener};

use super::{authorization::Token, state::ApiState};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterCourseListener {
    pub course: String,
    pub class: String,
    pub year: Option<i32>,
    pub webhook_url: String,
}
pub async fn register_course_listener(
    State(mut api_state): State<ApiState>,
    token: Token,
    Json(body): Json<RegisterCourseListener>,
) -> impl IntoResponse {
    let course_listener = CourseListener::new(
        crate::models::listener::Listener {
            id: token.listener_id,
        },
        body.course,
        body.class,
        body.year.unwrap_or(time::current_school_year()),
        body.webhook_url,
    );
    let course_listener =
        crate::logic::course_listener::register(&mut api_state.redis_db, &course_listener).await;

    match course_listener {
        true => (StatusCode::OK, Json(course_listener)).into_response(),
        false => StatusCode::CONFLICT.into_response(),
    }
}
