mod app;
mod config;
mod data;
mod error_response;

use config::config::Config;
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    app::app::start(&Config::new()).await;
}
