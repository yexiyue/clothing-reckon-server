use crate::{error::AppError, jwt::Claims, state::AppState};
use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use bcrypt::{verify, DEFAULT_COST};
use serde::Deserialize;
use serde_json::json;
use service::user::CreateUserParams;

pub fn route() -> Router<AppState> {
    Router::new()
        .route("/user/login", post(login))
        .route("/user", post(create).delete(logoff))
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

    if verify(params.password, &user.password)? {
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

async fn create(
    State(AppState { db, .. }): State<AppState>,
    Json(mut params): Json<CreateUserParams>,
) -> Result<impl IntoResponse, AppError> {
    params.password = bcrypt::hash(params.password, DEFAULT_COST)?;

    let user = service::user::UserService::create(&db, params).await?;

    Ok(Json(json!({
        "id": user.id,
        "username": user.username,
        "phone_number": user.phone_number,
        "create_at": user.create_at,
    })))
}

async fn logoff(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
) -> Result<impl IntoResponse, AppError> {
    let user = service::user::UserService::delete(&db, user_id).await?;

    Ok(Json(json!({
        "id": user.id,
        "username": user.username,
        "phone_number":user.phone_number,
        "create_at": user.create_at,
    })))
}
