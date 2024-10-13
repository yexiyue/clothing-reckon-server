use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use service::{
    procurement::{CreateProcurementParams, ProcurementService, UpdateProcurementParams},
    ListQueryParams,
};

use crate::{error::AppError, jwt::Claims, state::AppState};

pub fn route() -> Router<AppState> {
    Router::new()
        .route("/procurement", post(create).get(find))
        .route(
            "/procurement/:id",
            get(find_by_id).put(update).delete(delete),
        )
}

async fn create(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
    Json(params): Json<CreateProcurementParams>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(
        ProcurementService::create(&db, user_id, params).await?,
    ))
}

async fn delete(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(ProcurementService::delete(&db, user_id, id).await?))
}

async fn update(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
    Path(id): Path<i32>,
    Json(params): Json<UpdateProcurementParams>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(
        ProcurementService::update(&db, user_id, id, params).await?,
    ))
}

async fn find_by_id(
    State(AppState { db, .. }): State<AppState>,
    Path(id): Path<i32>,
    Claims { user_id, .. }: Claims,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(
        ProcurementService::find_by_id(&db, user_id, id).await?,
    ))
}

// 查找当前用户的进货记录
async fn find(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
    Query(params): Query<ListQueryParams>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(
        ProcurementService::find_by_user_id(&db, user_id, params).await?,
    ))
}
