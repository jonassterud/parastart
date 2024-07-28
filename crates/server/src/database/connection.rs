use crate::error::ServerError;
use sqlx::{postgres::PgPoolOptions, Connection, PgConnection, PgPool};

/// Create a connection pool to the database.
pub async fn pool() -> Result<PgPool, ServerError> {
    let url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new().connect(&url).await?;

    sqlx::migrate!("src/database/migrations").run(&pool).await?;

    Ok(pool)
}

/// Create a single connection to the database.
pub async fn single() -> Result<PgConnection, ServerError> {
    let url = std::env::var("DATABASE_URL")?;
    let conn = PgConnection::connect(&url).await?;

    Ok(conn)
}
