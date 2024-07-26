use super::version::Version;
use crate::{
    database::models::{self, Takeoff},
    error::ServerError,
};
use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use sqlx::PgPool;
use std::time::SystemTime;

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
    let description = data.value.description;
    let image = data.value.image;
    let latitude = data.value.latitude;
    let longitude = data.value.longitude;
    let creation = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs() as i64;

    sqlx::query!(
        r#"
            INSERT INTO takeoffs(description, image, latitude, longitude, creation)
            VALUES ($1, $2, $3, $4, $5)
        "#,
        description,
        image,
        latitude,
        longitude,
        creation
    )
    .execute(&*pool)
    .await?;

    Ok(())
}
