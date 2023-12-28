pub mod data {
    use sqlx::{postgres::PgPoolOptions, Postgres, Pool};
    use tracing::error;
    #[derive(Clone)]
    pub struct AppState {
        pub pool: Pool<Postgres>,
    }
    use crate::config::config::Config;
    impl AppState{
        pub async fn new(config: &Config) -> AppState{
            let connect = PgPoolOptions::new()
            .max_connections(10)
            .connect(&config.database.uri).await;
            match connect{
                Ok(db) => {
                    AppState{pool:db}
                },
                Err(err) => {
                    let error = format!("data - connection - can't connect to database | {}", err);
                    error!(error);
                    panic!("{}", error);
                },
            }
    
        }
    }
}
