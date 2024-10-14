use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use service::production::{
    CreateProductionParams, ProductionListQueryParams, ProductionService, UpdateProductionParams,
};

use crate::{error::AppError, jwt::Claims, state::AppState};

pub fn route() -> Router<AppState> {
    Router::new()
        .route("/production", post(create).get(find))
        .route(
            "/production/:id",
            get(find_by_id).put(update).delete(delete),
        )
        .route("/production/:id/settle", post(settle))
}

async fn create(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
    Json(params): Json<CreateProductionParams>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(ProductionService::create(&db, user_id, params).await?))
}

async fn delete(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(ProductionService::delete(&db, user_id, id).await?))
}

async fn update(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
    Path(id): Path<i32>,
    Json(params): Json<UpdateProductionParams>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(
        ProductionService::update(&db, user_id, id, params).await?,
    ))
}

async fn settle(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(ProductionService::settle(&db, user_id, id).await?))
}

async fn find_by_id(
    State(AppState { db, .. }): State<AppState>,
    Path(id): Path<i32>,
    Claims { user_id, .. }: Claims,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(ProductionService::find_by_id(&db, user_id, id).await?))
}

async fn find(
    State(AppState { db, .. }): State<AppState>,
    Claims { user_id, .. }: Claims,
    Query(ProductionListQueryParams {
        list_query,
        staff_ids,
    }): Query<ProductionListQueryParams>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(
        ProductionService::find_by_user_id(&db, user_id, list_query, staff_ids).await?,
    ))
}
