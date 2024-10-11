use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub struct AnyhowError(anyhow::Error);

impl IntoResponse for AnyhowError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "code":StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            "reason":format!("Something went wrong: {}", self.0)
        }));

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

impl<E> From<E> for AnyhowError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[derive(Debug)]
pub enum AuthError {
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    ExpiredSignature,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::ExpiredSignature => (StatusCode::UNAUTHORIZED, "Expired signature"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "code": status.as_u16(),
            "reason": error_message,
        }));
        (status, body).into_response()
    }
}

pub enum AppError {
    AuthError(AuthError),
    AnyhowError(AnyhowError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::AuthError(e) => e.into_response(),
            AppError::AnyhowError(e) => e.into_response(),
        }
    }
}

impl From<AuthError> for AppError {
    fn from(e: AuthError) -> Self {
        AppError::AuthError(e)
    }
}

impl From<AnyhowError> for AppError {
    fn from(e: AnyhowError) -> Self {
        AppError::AnyhowError(e)
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::AnyhowError(AnyhowError(err.into()))
    }
}
