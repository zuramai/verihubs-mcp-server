use rmcp::transport::{streamable_http_server::session::local::LocalSessionManager, StreamableHttpServerConfig, StreamableHttpService};
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

use crate::service::config::VerihubsConfig;

mod error;
mod model;
mod service;

const BIND_ADDRESS: &str = "0.0.0.0:8005";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_env_filter(
            EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into())
        )
        .init();
    
    tracing::info!("Starting Verihubs MCP server");

    let app_id = std::env::var("VERIHUBS_APP_ID").expect("VERIHUBS_APP_ID env missing");
    let api_key = std::env::var("VERIHUBS_API_KEY").expect("VERIHUBS_API_KEY env missing");
    let config = VerihubsConfig::new(api_key, app_id);

    let service = StreamableHttpService::new(
        move || Ok(service::VerihubsService::new(config.clone())), 
        LocalSessionManager::default().into(), 
        Default::default()
    );

    let router = axum::Router::new().nest_service("/mcp", service);
    let tcp_listener = TcpListener::bind(BIND_ADDRESS).await.expect(format!("Failed to listen to {}", BIND_ADDRESS).as_str());
    let _ = axum::serve(tcp_listener, router)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.unwrap()
        }).await;

}
