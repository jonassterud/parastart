use std::time::Duration;
use sqlx::{postgres::PgPoolOptions, PgPool};
use crate::error::ServerError;

pub async fn get_pool() -> Result<PgPool, ServerError> {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost".to_string());
    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(3))
        .connect(&url).await?;

    Ok(pool)
}
