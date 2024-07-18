mod error;
mod handler;

use axum::{
    routing::get,
    Router,
};
use error::ServerError;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    // Set up tracing subscriber
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create router and listener
    let server = Router::new()
        .route("/", get(handler::root))
        .layer(TraceLayer::new_for_http());
    let listener = TcpListener::bind("0.0.0.0:5050").await?;

    // Start listening
    tracing::debug!("listening on http://{}", listener.local_addr()?);
    axum::serve(listener, server).await?;

    Ok(())
}
