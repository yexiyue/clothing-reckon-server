use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::json;

use crate::{error::AppError, jwt::Claims, state::AppState};

pub fn route() -> Router<AppState> {
    Router::new().route("/user/login", post(login))
}

#[derive(Debug, Deserialize)]
struct LoginParams {
    phone_number: String,
    password: String,
}

async fn login(
    State(state): State<AppState>,
    Json(params): Json<LoginParams>,
) -> Result<impl IntoResponse, AppError> {
    let user =
        service::user::UserService::find_by_phone_number(&state.db, params.phone_number).await?;
    if user.password != params.password {
        return Err(anyhow::anyhow!("密码错误").into());
    }
    let claims = Claims::new(user.id);

    Ok(Json(json!({
        "token": claims.encode(&state.jwt_secret)?,
        "user": {
            "id": user.id,
            "username": user.username,
            "phone_number": user.phone_number,
            "create_at": user.create_at,
        },
    })))
}
