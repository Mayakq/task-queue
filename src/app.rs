pub mod app{
    use crate::{config::config::Config, data};
    
    use axum::{Router,  http::StatusCode, routing::get};
    async fn healthy() -> StatusCode{
        StatusCode::OK
    }
    pub async fn start(config: &Config){
        let app = Router::new()
        .route("/", get(healthy))
        .with_state(data::data::AppState::new(config).await.pool);
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}