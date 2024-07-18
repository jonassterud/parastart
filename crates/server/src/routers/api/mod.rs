use axum::{
    Router,
    response::Response,
    middleware::{self, Next},
    extract::Request,
};

use crate::{database, error::ServerError};

const VERSION: usize = 0;

pub async fn router() -> Result<Router, ServerError> {
    Ok(Router::new().nest(&format!("/v{VERSION}"), current().await?))
}

async fn current() -> Result<Router, ServerError> {
    Ok(Router::new())
}
