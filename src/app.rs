pub mod app{
    use crate::config;
    
    use axum::{Router, response::Response, http::StatusCode, routing::get};
    use config::config::{Config};
    async fn healthy() -> (StatusCode){
        StatusCode::OK
    }
    pub async fn start(config: &Config){
        let app = Router::new()
        .route("/", get(healthy));
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}