pub mod app {
    use crate::{config, data::data};
    use tracing::Level;
    use axum::{http::StatusCode, routing::get, Router};
    use config::config::Config;
    async fn healthy() -> StatusCode {
        StatusCode::OK
    }
    pub async fn start(config: &Config) {
        let layer = tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            .init();
        let state = data::connect(config).await;
        let app = Router::new()
        .with_state(state)
        .route("/", get(healthy))
        .layer(layer);
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}
