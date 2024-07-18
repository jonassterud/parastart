mod root;

use axum::{routing::get, Router};
use crate::error::ServerError;

pub async fn router() -> Result<Router, ServerError> {
    Ok(Router::new().route("/", get(root::get)))
}
