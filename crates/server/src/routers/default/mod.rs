use axum::{
    async_trait,
    extract::{FromRequestParts, Query, Request},
    http::request::Parts,
    routing::get,
    Router,
};
use serde::Deserialize;
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};

pub fn router() -> Router {
    Router::new()
        .nest_service("/scripts", ServeDir::new("crates/www/scripts"))
        .nest_service("/styles", ServeDir::new("crates/www/styles"))
        .nest_service("/assets", ServeDir::new("crates/www/assets"))
        .nest_service("/", ServeFile::new("crates/www/pages/home.html"))
        .nest_service("/map", ServeFile::new("crates/www/pages/map.html"))
        .nest_service(
            "/takeoffs",
            get(|params: Params, request: Request| async move {
                if params.id.is_some() {
                    ServeDir::new("crates/www/pages/takeoff.html")
                        .oneshot(request)
                        .await
                } else {
                    ServeDir::new("crates/www/pages/list.html")
                        .oneshot(request)
                        .await
                }
            }),
        )
}

#[derive(Default)]
struct Params {
    id: Option<i32>,
}

#[async_trait]
impl<S> FromRequestParts<S> for Params
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        #[derive(Deserialize)]
        struct FormatQuery {
            id: i32,
        }

        let Query(query) = match Query::<FormatQuery>::from_request_parts(parts, state).await {
            Ok(query) => query,
            Err(_) => return Ok(Self::default()),
        };

        Ok(Self { id: Some(query.id) })
    }
}
