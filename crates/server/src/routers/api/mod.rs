mod health;
mod takeoffs;
mod users;
mod version;

use axum::Router;

// RESTish
pub fn router() -> Router {
    Router::new()
        .merge(users::router())
        .merge(takeoffs::router())
        .merge(health::router())
}
