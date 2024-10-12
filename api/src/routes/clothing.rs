use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use service::clothing::{
    ClothingListQueryParams, ClothingService, CreateClothingParams, UpdateClothingParams,
};

use crate::{error::AppError, jwt::Claims, state::AppState};

pub fn route() -> Router<AppState> {
    Router::new()
        .route("/clothing", post(create).get(find_staffs))
        .route("/clothing/:id", get(find_by_id).put(update).delete(delete))
}

async fn create(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
    Json(params): Json<CreateClothingParams>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(ClothingService::create(&db, user_id, params).await?))
}

async fn delete(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(ClothingService::delete(&db, user_id, id).await?))
}

async fn update(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
    Path(id): Path<i32>,
    Json(params): Json<UpdateClothingParams>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(
        ClothingService::update(&db, user_id, id, params).await?,
    ))
}

async fn find_by_id(
    State(AppState { db, .. }): State<AppState>,
    Path(id): Path<i32>,
    Claims { user_id, .. }: Claims,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(ClothingService::find_by_id(&db, user_id, id).await?))
}

// 查找当前用户的所有老板列表
async fn find_staffs(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
    Query(params): Query<ClothingListQueryParams>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(
        ClothingService::find_by_user_id(&db, user_id, params).await?,
    ))
}
