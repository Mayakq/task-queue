use config::config::Config;

mod app;
mod config;
mod data;


#[tokio::main]
async fn main() {
    app::app::start(&Config::new()).await;
}
