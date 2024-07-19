use tower_http::services::ServeDir;
use axum::Router;
use crate::error::ServerError;

pub async fn router() -> Result<Router, ServerError> {
    let dir = ServeDir::new("crates/www");
    let router = Router::new().nest_service("/", dir);

    Ok(router)
}
