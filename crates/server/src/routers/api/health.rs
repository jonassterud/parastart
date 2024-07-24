use anyhow::anyhow;
use axum::{routing::get, Extension, Router};
use sqlx::PgPool;

use super::version::Version;
use crate::error::ServerError;

pub fn router() -> Router {
    Router::new().route("/api/:version/health", get(get_health))
}

async fn get_health(version: Version, pool: Extension<PgPool>) -> Result<String, ServerError> {
    if let Ok(_) = pool.acquire().await {
        Ok(format!("OK\n\nVersion: {:?}\nDatabase: OK", version))
    } else {
        Err(ServerError::INTERNAL_SERVER_ERROR(anyhow!(
            "failed connecting to database"
        )))
    }
}
