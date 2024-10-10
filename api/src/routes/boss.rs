use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use service::{
    boss::{BossService, CreateBossParams, UpdateBossParams},
    ListQueryParams,
};

use crate::{error::AppError, jwt::Claims, state::AppState};

pub fn route() -> Router<AppState> {
    Router::new()
        .route("/boss", post(create).get(find_bosses))
        .route("/boss/:id", get(find_by_id).put(update).delete(delete))
}

async fn create(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
    Json(params): Json<CreateBossParams>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(BossService::create(&db, user_id, params).await?))
}

async fn delete(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(BossService::delete(&db, user_id, id).await?))
}

async fn update(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
    Path(id): Path<i32>,
    Json(params): Json<UpdateBossParams>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(BossService::update(&db, user_id, id, params).await?))
}

async fn find_by_id(
    State(AppState { db, .. }): State<AppState>,
    Path(id): Path<i32>,
    _claims: Claims,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(BossService::find_by_id(&db, id).await?))
}

// 查找当前用户的所有老板列表
async fn find_bosses(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
    Query(params): Query<ListQueryParams>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(
        BossService::find_by_user_id(&db, user_id, params).await?,
    ))
}
