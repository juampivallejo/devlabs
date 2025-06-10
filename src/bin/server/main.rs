use api_lib::{
    config::Config,
    domain::finance::service::Service,
    inbound::http::{HttpServer, HttpServerConfig},
    outbound::{email_client::EmailClient, postgres::Postgres, prometheus::Prometheus},
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

    let postgres = Postgres::new(&config.database_url).await?;
    let prometheus = Prometheus::new();
    let email_client = EmailClient::new();
    let finance_service = Service::new(postgres, prometheus, email_client);

    let server_config = HttpServerConfig {
        port: &config.server_port,
    };
    tracing::info!("Starting server with server config: {:?}", server_config);
    let http_server = HttpServer::new(finance_service, server_config).await?;
    http_server.run().await
}
