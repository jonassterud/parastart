use axum::Router;
use tower_http::services::ServeDir;

pub fn router() -> Router {
    Router::new().nest_service("/", ServeDir::new("crates/www"))
}
