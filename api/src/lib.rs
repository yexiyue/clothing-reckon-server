use axum::Router;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use state::AppState;
use std::time::Duration;
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer};

mod error;
mod jwt;
mod routes;
mod state;

pub async fn router(db: DatabaseConnection, jwt_secret: String) -> anyhow::Result<Router> {
    Migrator::up(&db, None).await?;

    let router = Router::new()
        .merge(routes::user::route())
        .merge(routes::boss::route())
        .merge(routes::staff::route())
        .merge(routes::clothing::route())
        .merge(routes::procurement::route())
        .merge(routes::shipment::route())
        .with_state(AppState::new(db, jwt_secret))
        .layer(TimeoutLayer::new(Duration::from_secs(5)))
        .layer(CorsLayer::permissive());
    Ok(router)
}
