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
    let name = data.value.name;
    let description = data.value.description;
    let image = data.value.image;
    let region = data.value.region;
    let altitude = data.value.altitude;
    let altitude_diff = data.value.altitude_diff;
    let latitude = data.value.latitude;
    let longitude = data.value.longitude;
    let wind_dirs = &data.value.wind_dirs;
    let info_url = data.value.info_url;
    let source_url = data.value.source_url;
    let created = data.value.created;
    let updated = data.value.updated;

    sqlx::query!(
        r#"
            INSERT INTO takeoffs(name, description, image, region, altitude, altitude_diff, latitude, longitude, wind_dirs, info_url, source_url, created, updated)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        "#,
        name,
        description,
        image,
        region,
        altitude,
        altitude_diff,
        latitude,
        longitude,
        wind_dirs,
        info_url,
        source_url,
        created,
        updated,
    )
    .execute(&*pool)
    .await?;

    Ok(())
}
