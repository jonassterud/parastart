mod database;
mod error;
mod routers;

pub use database::connection;
pub use database::helpers;
pub use database::models;
pub use error::ServerError;

use axum::{Extension, Router};
use rand::{RngCore, SeedableRng};
use rand_chacha::{rand_core::OsRng, ChaCha8Rng};
use std::sync::{Arc, Mutex};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub async fn run() -> Result<(), ServerError> {
    // Set up tracing subscriber
    const DEFAULT_FILTER: &str = "server=debug,tower_http=debug,axum::rejection=trace";
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| DEFAULT_FILTER.into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create listener and router
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5050").await?;
    let random = Arc::new(Mutex::new(ChaCha8Rng::seed_from_u64(OsRng.next_u64())));
    let pool = database::connection::pool().await?;
    let app = Router::new()
        .merge(routers::default::router())
        .merge(routers::api::router())
        .layer(CorsLayer::very_permissive()) // TODO
        .layer(TraceLayer::new_for_http())
        .layer(Extension(pool))
        .layer(Extension(random));

    // Start listening
    tracing::debug!("listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
