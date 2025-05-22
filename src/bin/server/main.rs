use api_lib::{
    config::Config,
    inbound::http::{HttpServer, HttpServerConfig},
    outbound::sqlite::Sqlite,
};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Default to INFO if RUST_LOG is not set
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::from_env()?;
    tracing::info!("Starting server with config: {:?}", config);
    let post_repo = Sqlite::new(&config.database_url).await?;

    let server_config = HttpServerConfig {
        port: &config.server_port,
    };
    tracing::info!("Starting server with server config: {:?}", server_config);
    let http_server = HttpServer::new(post_repo, server_config).await?;
    http_server.run().await
}
