use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use service::{
    staff::{CreateStaffParams, StaffService, UpdateStaffParams},
    ListQueryParams,
};

use crate::{error::AppError, jwt::Claims, state::AppState};

pub fn route() -> Router<AppState> {
    Router::new()
        .route("/staff", post(create).get(find))
        .route("/staff/:id", get(find_by_id).put(update).delete(delete))
}

async fn create(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
    Json(params): Json<CreateStaffParams>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(StaffService::create(&db, user_id, params).await?))
}

async fn delete(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(StaffService::delete(&db, user_id, id).await?))
}

async fn update(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
    Path(id): Path<i32>,
    Json(params): Json<UpdateStaffParams>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(StaffService::update(&db, user_id, id, params).await?))
}

async fn find_by_id(
    State(AppState { db, .. }): State<AppState>,
    Path(id): Path<i32>,
    Claims { user_id, .. }: Claims,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(StaffService::find_by_id(&db, user_id, id).await?))
}

// 查找当前用户的所有老板列表
async fn find(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
    Query(params): Query<ListQueryParams>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(
        StaffService::find_by_user_id(&db, user_id, params).await?,
    ))
}
