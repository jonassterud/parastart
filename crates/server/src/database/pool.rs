use crate::error::ServerError;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

use super::connection::ConnectionPool;

pub async fn get_pool() -> Result<ConnectionPool, ServerError> {
    let config = tokio_postgres::Config::new()
        .user("postgres")
        .password("postgres")
        .dbname("postgres")
        .host("0.0.0.0")
        .to_owned();
    let manager = PostgresConnectionManager::new(config, NoTls);
    let pool = Pool::builder().build(manager).await?;

    Ok(pool)
}
