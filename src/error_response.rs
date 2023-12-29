pub mod error_response {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct ErrResponse {
        pub message: String,
    }

    impl IntoResponse for ErrResponse {
        fn into_response(self) -> axum::response::Response {
            if self.message.len() > 0 {
                return (StatusCode::BAD_REQUEST, self.message).into_response()
            }
            (StatusCode::BAD_REQUEST).into_response()
        }
    }

    impl ErrResponse {
        pub async fn new(message: String) -> Self {
            Self {
                message
            }
        }
    }
}