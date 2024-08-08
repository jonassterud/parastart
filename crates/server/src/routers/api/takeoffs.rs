use super::version::Version;
use crate::{
    database::helpers,
    error::ServerError,
    models::{Data, GetTakeoff, NewTakeoff},
};
use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::extract::Query;
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
    fields: Vec<String>,
    count: bool,
}

impl Default for GetTakeoffsParams {
    fn default() -> Self {
        Self {
            id: None,
            page: 1,
            limit: i64::MAX,
            region: "%".to_owned(),
            fields: Vec::default(),
            count: false,
        }
    }
}

async fn get_takeoffs(
    version: Version,
    pool: Extension<PgPool>,
    Query(params): Query<GetTakeoffsParams>,
) -> Result<Json<Vec<GetTakeoff>>, ServerError> {
    let fields = params.fields.join(", ");
    let fields = if fields.is_empty() { "*".to_owned() } else { fields };

    // TODO: bind on fields?
    let out = if let Some(id) = params.id {
        sqlx::query_as(&format!("SELECT {fields} FROM takeoffs WHERE id = $1"))
            .bind(id)
            .fetch_optional(&*pool)
            .await?
            .map_or(Vec::new(), |v| vec![v])
    } else {
        sqlx::query_as(&format!(
            "SELECT {fields} FROM takeoffs WHERE region LIKE $1 LIMIT $2 OFFSET $3"
        ))
        .bind(params.region)
        .bind(params.limit)
        .bind((params.page - 1) * params.limit)
        .fetch_all(&*pool)
        .await?
    };

    Ok(Json(out))
}

async fn post_takeoffs(
    version: Version,
    pool: Extension<PgPool>,
    Json(data): Json<Data<NewTakeoff>>,
) -> Result<(), ServerError> {
    helpers::insert_takeoff(&*pool, &data.value).await?;

    Ok(())
}
