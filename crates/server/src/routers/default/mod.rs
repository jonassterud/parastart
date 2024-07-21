use crate::{database::ConnectionPool, error::ServerError};
use axum::Router;
use tower_http::services::ServeDir;

pub async fn router() -> Result<Router<ConnectionPool>, ServerError> {
    let dir = ServeDir::new("crates/www");
    let router = Router::new().nest_service("/", dir);

    Ok(router)
}
