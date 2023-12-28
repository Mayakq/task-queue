pub mod data {
    use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tracing::error;

    #[derive(Clone)]
    pub struct AppState {
        pub pool: Arc<Mutex<Pool<Postgres>>>,
    }

    use crate::config::config::Config;

    impl AppState {
        pub async fn new(config: &Config) -> Self {
            let connect = PgPoolOptions::new()
                .max_connections(10)
                .connect(&config.database.uri)
                .await;
            match connect {
                Ok(db) => Self {
                    pool: Arc::new(Mutex::new(db)),
                },
                Err(err) => {
                    let error = format!("data - connection - can't connect to database | {}", err);
                    error!(error);
                    panic!("{}", error);
                }
            }
        }
    }
}
