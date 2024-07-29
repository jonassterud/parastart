use super::version::Version;
use crate::{
    database::{
        helpers,
        models::{self, Takeoff},
    },
    error::ServerError,
};
use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use sqlx::PgPool;

pub fn router() -> Router {
    Router::new()
        .route("/api/:version/takeoffs", get(get_takeoffs))
        .route("/api/:version/takeoffs", post(post_takeoffs))
}

async fn get_takeoffs(
    version: Version,
    pool: Extension<PgPool>,
) -> Result<Json<Vec<Takeoff>>, ServerError> {
    // TODO: Oops, memory
    let takeoffs: Vec<Takeoff> = sqlx::query_as("SELECT * FROM takeoffs")
        .fetch_all(&*pool)
        .await?;

    Ok(Json(takeoffs))
}

async fn post_takeoffs(
    version: Version,
    pool: Extension<PgPool>,
    Json(data): Json<models::Data<models::NewTakeoff>>,
) -> Result<(), ServerError> {
    helpers::insert_takeoff(&*pool, &data.value).await?;

    Ok(())
}
