use super::version::Version;
use crate::{error::ServerError, models::{Takeoff, CreateTakeoff}};
use axum::{Json, extract::State};
use sqlx::PgPool;
use tracing::info;

pub async fn get(
    version: Version,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Takeoff>>, ServerError> {
    // TODO: Oops, memory
    let takeoffs: Vec<Takeoff> = sqlx::query_as("SELECT * FROM takeoffs").fetch_all(&pool).await?;
    
    Ok(Json(takeoffs))
}

pub async fn put(
    version: Version,
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTakeoff>,
) -> Result<String, ServerError> {
    info!("{:?}", payload);

    Ok(format!("OK\n\nVersion: {:?}", version))
}
