pub mod auth;
pub mod models;

use crate::error::ServerError;
use sqlx::{postgres::PgPoolOptions, PgPool};

/// Create a connection pool to the database.
pub async fn create_connection_pool() -> Result<PgPool, ServerError> {
    let url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new().connect(&url).await?;

    sqlx::migrate!("src/database/migrations").run(&pool).await?;

    Ok(pool)
}
