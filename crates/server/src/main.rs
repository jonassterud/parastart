mod database;
mod error;
mod routers;

use axum::{routing::get, Router};
use error::ServerError;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    // Set up tracing subscriber
    const DEFAULT_FILTER: &str = "server=debug,tower_http=debug,axum::rejection=trace";
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| DEFAULT_FILTER.into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create router and listener
    let server = Router::new()
        .with_state(database::get_pool().await?)
        .layer(TraceLayer::new_for_http())
        //.nest("/", routers::default::router().await?)
        .nest("/api", routers::api::router().await?);
    let listener = TcpListener::bind("0.0.0.0:5050").await?;

    // Start listening
    tracing::debug!("listening on http://{}", listener.local_addr()?);
    axum::serve(listener, server).await?;

    Ok(())
}
