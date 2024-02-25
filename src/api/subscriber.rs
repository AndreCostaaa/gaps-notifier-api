use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    logic::time,
    models::{spy::Spy, subscriber::Subscriber},
};

use super::{authorization::Token, state::ApiState};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterSpy {
    pub webhook_url: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterSubscriber {
    pub course: String,
    pub class: String,
    pub year: Option<i32>,
    pub webhook_url: String,
}
pub async fn register_course_subscriber(
    State(mut api_state): State<ApiState>,
    token: Token,
    Json(body): Json<RegisterSubscriber>,
) -> impl IntoResponse {
    let subscriber = Subscriber::new(
        crate::models::user::User { id: token.user_id },
        body.course,
        body.class,
        body.year.unwrap_or(time::current_school_year()),
        body.webhook_url,
    );
    let created = crate::logic::subscriber::register(&mut api_state.redis_db, &subscriber).await;

    match created {
        true => return (StatusCode::OK, Json(subscriber)).into_response(),
        false => return StatusCode::CONFLICT.into_response(),
    }
}

pub async fn register_spy(
    State(mut api_state): State<ApiState>,
    token: Token,
    Json(body): Json<RegisterSpy>,
) -> impl IntoResponse {
    let spy = Spy::new(
        crate::models::user::User { id: token.user_id },
        body.webhook_url,
    );
    let created = crate::logic::spy::register_spy(&mut api_state.redis_db, &spy).await;
    match created {
        true => return (StatusCode::OK, Json(spy)).into_response(),
        false => return StatusCode::CONFLICT.into_response(),
    }
}

pub async fn get_spies(State(mut api_state): State<ApiState>, token: Token) -> impl IntoResponse {
    if !token.is_admin {
        return StatusCode::FORBIDDEN.into_response();
    }

    let spies = crate::logic::spy::get_spies(&mut api_state.redis_db).await;
    return (StatusCode::OK, Json(spies)).into_response();
}
pub async fn delete_spy(
    State(mut api_state): State<ApiState>,
    token: Token,
    spy: Json<Spy>,
) -> impl IntoResponse {
    if !token.is_admin {
        return StatusCode::FORBIDDEN.into_response();
    }

    let deleted = crate::logic::spy::delete_spy(&mut api_state.redis_db, &spy).await;
    match deleted {
        true => return StatusCode::OK.into_response(),
        false => return StatusCode::NOT_FOUND.into_response(),
    }
}
