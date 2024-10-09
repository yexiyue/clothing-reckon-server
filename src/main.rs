use sea_orm::SqlxPostgresConnector;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
    let router = api::router(db, "yexiyue666".into()).await?;
    Ok(router.into())
}
