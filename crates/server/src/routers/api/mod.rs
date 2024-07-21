mod health;
mod takeoffs;
mod version;

use sqlx::PgPool;
use crate::error::ServerError;
use axum::{routing::get, Router};

pub async fn router() -> Result<Router<PgPool>, ServerError> {
    let router = Router::new()
        .route("/:version/health", get(health::get))
        .route("/:version/takeoffs", get(takeoffs::get));

    Ok(router)
}
