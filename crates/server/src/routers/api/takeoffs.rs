use super::version::Version;
use crate::{
    database::models::{NewTakeoff, Takeoff},
    error::ServerError,
};
use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use sqlx::PgPool;
use tracing::info;

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
    Json(payload): Json<NewTakeoff>,
) -> Result<String, ServerError> {
    // let id: i32 = sqlx::query!(r#"
    //         INSERT INTO takeoffs(body, picture, latitude, longitude, creation)
    //         VALUES ($1, $2, $3, $4, $5)
    //         RETURNING id
    //     "#, payload.body, payload.picture, payload.latitude, payload.longitude).fetch_one(&pool);

    info!("{:?}", payload);

    Ok(format!("OK\n\nVersion: {:?}", version))
}
