use axum::{Router, routing::get};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Default to INFO if RUST_LOG is not set
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::debug!("Setup logging with filter");
    let app = Router::new();

    tracing::info!("Starting server on 0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");

    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("Server error: {}", e);
    }
}
