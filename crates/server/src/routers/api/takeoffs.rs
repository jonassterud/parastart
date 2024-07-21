use super::version::Version;
use crate::{database::DatabaseConnection, error::ServerError, models::Takeoff};
use axum::Json;
use bb8_postgres::tokio_postgres::Row;
use tracing::info;

pub async fn get(version: Version, DatabaseConnection(conn): DatabaseConnection) -> Result<(), ServerError> {
    let rows = conn.query("SELECT * FROM takeoffs", &[]).await?;
    info!("{:?}", rows);

    Ok(())
}

pub async fn put(
    version: Version,
    DatabaseConnection(conn): DatabaseConnection,
) -> Result<String, ServerError> {
    Ok(format!("OK\n\nVersion: {:?}", version))
}
