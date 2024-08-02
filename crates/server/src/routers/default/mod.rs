use axum::{response::Redirect, routing::get, Router};
use tower_http::services::ServeDir;

pub fn router() -> Router {
    Router::new()
    .nest_service("/", ServeDir::new("crates/www"))
    //.route("/", get(|| async { Redirect::permanent("/home") }))
}
