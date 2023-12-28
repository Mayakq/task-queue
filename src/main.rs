mod config;
mod data;
mod app;
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    app::app::start(&config::config::init_config().unwrap()).await;
}
