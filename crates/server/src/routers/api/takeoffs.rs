use super::version::Version;
use crate::{
    database::{
        helpers,
        models::{self, Takeoff},
    },
    error::ServerError,
};
use axum::{
    extract::Query,
    routing::{get, post},
    Extension, Json, Router,
};
use serde::Deserialize;
use sqlx::PgPool;

pub fn router() -> Router {
    Router::new()
        .route("/api/:version/takeoffs", get(get_takeoffs))
        .route("/api/:version/takeoffs", post(post_takeoffs))
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct GetTakeoffsParams {
    id: Option<i32>,
    page: i64,
    limit: i64,
    region: String,
}

impl Default for GetTakeoffsParams {
    fn default() -> Self {
        Self {
            id: None,
            page: 1,
            limit: 10,
            region: "%".to_owned(),
        }
    }
}

async fn get_takeoffs(
    version: Version,
    pool: Extension<PgPool>,
    Query(params): Query<GetTakeoffsParams>,
) -> Result<Json<Vec<Takeoff>>, ServerError> {
    let out = if params.id.is_some() {
        sqlx::query_as!(
            Takeoff,
            r#"SELECT * FROM takeoffs WHERE ID = $1"#,
            params.id.unwrap(),
        )
        .fetch_all(&*pool)
        .await?
    } else {
        sqlx::query_as!(
            Takeoff,
            r#"SELECT * FROM takeoffs WHERE region LIKE $1 LIMIT $2 OFFSET $3"#,
            params.region,
            params.limit,
            (params.page - 1) * params.limit
        )
        .fetch_all(&*pool)
        .await?
    };

    Ok(Json(out))
}

async fn post_takeoffs(
    version: Version,
    pool: Extension<PgPool>,
    Json(data): Json<models::Data<models::NewTakeoff>>,
) -> Result<(), ServerError> {
    helpers::insert_takeoff(&*pool, &data.value).await?;

    Ok(())
}
