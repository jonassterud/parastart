use anyhow::anyhow;
use axum::extract::State;
use sqlx::PgPool;

use super::version::Version;
use crate::error::ServerError;

pub async fn get(version: Version, State(pool): State<PgPool>) -> Result<String, ServerError> {
    if let Ok(_) = pool.acquire().await {
        Ok(format!("OK\n\nVersion: {:?}\nDatabase: OK", version))
    } else {
        Err(ServerError::new(anyhow!("failed connecting to database")))
    }
    
}
