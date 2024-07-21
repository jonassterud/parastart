use crate::error::ServerError;
use sqlx::PgPool;
use axum::Router;
use tower_http::services::ServeDir;

pub async fn router() -> Result<Router<PgPool>, ServerError> {
    let dir = ServeDir::new("crates/www");
    let router = Router::new().nest_service("/", dir);

    Ok(router)
}
