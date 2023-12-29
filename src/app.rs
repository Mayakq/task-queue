pub mod app {
    use crate::{config::config::Config};

    use crate::data::data::AppState;
    use axum::extract::{Json, State};
    use axum::http::StatusCode;
    use axum::response::{IntoResponse, Response};
    use axum::routing::{get, post};
    use axum::Router;
    use serde::{Deserialize, Serialize};

    use sqlx::{query, Executor, query_as};
    use crate::error_response::error_response::ErrResponse;


    #[derive(Deserialize, Serialize)]
    struct CreateTask {
        worker: String,
        text: String,
    }

    async fn add_tasks(
        State(state): State<AppState>,
        Json(payload): Json<CreateTask>,
    ) -> impl IntoResponse {
        let query = query(
            r#"
            insert into tasks (text, worker)
            values ($1, (select id from worker where name = $2));"#,
        )
            .bind(payload.text)
            .bind(payload.worker);
        let result = state.pool.lock().await.execute(query).await;
        return match result {
            Ok(_) => {
                (StatusCode::CREATED, "successful").into_response()
            }
            Err(_err) => {
                ErrResponse::new("error response".to_string()).await.into_response()
            }
        };
        // todo(create normal response message)
    }

    #[derive(Deserialize, Serialize)]
    struct Tasks {
        text: String,
        task: uuid::Uuid,
        worker: String,
    }

    async fn get_all_tasks(State(state): State<AppState>) -> Response {
        let query =
            query_as!(Tasks,
                "select text, task, worker.name as worker
                from tasks
         join worker
              on worker.id = worker")
                .fetch_all(&*state.pool.lock().await).await;
        return match query {
            Ok(vec) => {
                (StatusCode::ACCEPTED, Json(vec)).into_response()
            }
            Err(_) => {
                return StatusCode::BAD_REQUEST.into_response();
            }
        };
    }

    pub async fn start(config: &Config) {
        let app = Router::new()
            .route("/task", post(add_tasks))
            .route("/tasks", get(get_all_tasks))
            .with_state(AppState::new(config).await);
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}
