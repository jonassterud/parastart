use super::models::NewTakeoff;
use sqlx::{Executor, Postgres};

pub async fn insert_takeoff<'a, E>(executor: E, data: &NewTakeoff) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
    sqlx::query!(
        r#"
            INSERT INTO takeoffs(name, description, image, region, altitude, altitude_diff, latitude, longitude, wind_dirs, info_url, source_url, created, updated)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        "#,
        data.name,
        data.description,
        data.image,
        data.region,
        data.altitude,
        data.altitude_diff,
        data.latitude,
        data.longitude,
        &data.wind_dirs,
        data.info_url,
        data.source_url,
        data.created,
        data.updated,
    )
    .execute(executor)
    .await?;

    Ok(())
}
