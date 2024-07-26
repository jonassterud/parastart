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
    let latitude = data.value.latitude;
    let longitude = data.value.longitude;
    let holfuy_url = data.value.holfuy_url;
    let wind_directions = &data.value.wind_directions;

    sqlx::query!(
        r#"
            INSERT INTO takeoffs(name, description, image, region, latitude, longitude, holfuy_url, wind_directions)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        name,
        description,
        image,
        region,
        latitude,
        longitude,
        holfuy_url,
        wind_directions
    )
    .execute(&*pool)
    .await?;

    Ok(())
}
