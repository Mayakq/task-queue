pub mod app {
    use crate::{config::config::Config, data};

    use crate::data::data::AppState;
    use axum::extract::{Json, State};
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use axum::routing::post;
    use axum::Router;
    use serde::{Deserialize, Serialize};
    
    use sqlx::{query, Executor};



    #[derive(Deserialize, Serialize)]
    struct CreateTask {
        manager: String,
        text: String,
    }

    async fn add_tasks(
        State(state): State<AppState>,
        Json(payload): Json<CreateTask>,
    ) -> impl IntoResponse {
        let query = query(
            r#"
            insert into tasks (text, worker)
            values ($1, (select id from manager where name = $2));"#,
        )
        .bind(payload.text)
        .bind(payload.manager);
        let result = state.pool.lock().await.execute(query).await;
        match result {
            Ok(_) => {
                return StatusCode::CREATED;
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
        return StatusCode::BAD_REQUEST;
    }

    pub async fn start(config: &Config) {
        let app = Router::new()
            .route("/task", post(add_tasks))
            .with_state(data::data::AppState::new(config).await);
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}
