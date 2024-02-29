use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;

use crate::logic;
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::state::ApiState;
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    access_token: String,
    token_type: String,
    expiration_timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenRequest {
    user_id: u128,
}
pub async fn get_token(
    State(mut api_state): State<ApiState>,
    body: Json<TokenRequest>,
) -> impl IntoResponse {
    match logic::user::get_user(&mut api_state.redis_db, body.user_id).await {
        Some(_) => match generate_token(body.user_id, false) {
            Ok(token) => {
                return (StatusCode::OK, Json(token)).into_response();
            }
            Err(e) => e.into_response(),
        },
        None => {
            return AuthError::InvalidUser.into_response();
        }
    }
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    InvalidUser,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),

            AuthError::InvalidUser => (StatusCode::NOT_FOUND, "Invalid user"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub user_id: u128,
    pub exp: i64,
    pub is_admin: bool,
}

#[async_trait]
impl<S> FromRequestParts<S> for Token
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, AuthError> {
        // Extract the token from the authorization header
        let auth = parts
            .headers
            .get("Authorization")
            .ok_or(AuthError::MissingCredentials)?;

        if !auth.to_str().unwrap().starts_with("Bearer ") {
            return Err(AuthError::WrongCredentials);
        }
        let token = auth.to_str().unwrap().split(" ").last();

        if token.is_none() {
            return Err(AuthError::WrongCredentials);
        }

        let token = token.unwrap();
        let admin_token = std::env::var("ADMIN_TOKEN");
        if let Ok(admin_token) = admin_token {
            if token == admin_token {
                return Ok(Token {
                    user_id: 0,
                    exp: 0,
                    is_admin: true,
                });
            }
        }
        //Decode token data
        let token_data = decode::<Token>(token, &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

fn generate_token(user_id: u128, is_admin: bool) -> Result<TokenResponse, AuthError> {
    const ONE_DAY: i64 = 60 * 60 * 24;
    const FIVE_YEARS: i64 = ONE_DAY * 365 * 5;
    let token = Token {
        user_id,
        is_admin,
        exp: if is_admin {
            chrono::Utc::now().timestamp() + ONE_DAY
        } else {
            chrono::Utc::now().timestamp() + FIVE_YEARS
        },
    };
    let access_token =
        encode(&Header::default(), &token, &KEYS.encoding).map_err(|_| AuthError::TokenCreation)?;

    Ok(TokenResponse {
        access_token: access_token,
        token_type: "Bearer".to_string(),
        expiration_timestamp: token.exp,
    })
}
