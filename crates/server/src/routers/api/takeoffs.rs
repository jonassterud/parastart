use super::version::Version;
use crate::{error::ServerError, models::Takeoff};
use axum::{Json, extract::State};
use sqlx::PgPool;

pub async fn get(
    version: Version,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Takeoff>>, ServerError> {
    let takeoffs: Vec<Takeoff> = sqlx::query_as("SELECT * FROM takeoffs").fetch_all(&pool).await?;
    
    Ok(Json(takeoffs))
}

pub async fn put(
    version: Version,
    State(pool): State<PgPool>,
) -> Result<String, ServerError> {
    Ok(format!("OK\n\nVersion: {:?}", version))
}
