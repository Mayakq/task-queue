mod app;
mod config;
mod data;
use config::config::Config;
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    app::app::start(&Config::new()).await;
}
