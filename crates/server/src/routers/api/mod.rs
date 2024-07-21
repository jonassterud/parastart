mod health;
mod takeoffs;
mod version;

use crate::models;
use crate::database::ConnectionPool;
use crate::error::ServerError;
use axum::{
    routing::get,
    Router,
};

pub async fn router() -> Result<Router<ConnectionPool>, ServerError> {
    let router = Router::new()
        .route("/:version/health", get(health::get))
        .route("/:version/takeoffs", get(takeoffs::get));

    Ok(router)
}
