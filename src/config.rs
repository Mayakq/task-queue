pub mod config {
    use serde::Deserialize;
    use tracing::{error, info};
    #[derive(Deserialize, Debug)]
    pub struct Config {
        #[serde(rename = "DB")]
        pub database: Database,
    }
#[derive(Deserialize, Debug)]
    pub struct Database {
        #[serde(rename = "DATABASE_URL")]
        pub uri: String,
        #[serde(rename = "MAX_POOL")]
        pub max_pool_connection: u8,
    }
    impl Config {
        /// Creates a new [`Config`].
        pub fn new(database: Database) -> Self {
            match init_config() {
                Ok(config) => config,
                Err(_) => todo!(),
            }
        }
    }

    pub fn init_config() -> Result<Config, String> {
        let toml = read_toml();
        return match toml {
            Ok(str) => {
                let config: Result<Config, toml::de::Error> = toml::from_str(str.as_str());
                match config {
                    Ok(config) => {
                        info!("init - init_config | Parse Toml config - {:#?}", config);
                        Ok(config)
                    }
                    Err(err) => {
                        error!("init - init_config | {}", err);
                        return Err(err.to_string());
                    }
                }
            }
            Err(error) => {
                let error = error.to_string();
                tracing::error!(error);
                Err(error)
            }
        };
    }
    fn read_toml() -> std::io::Result<String> {
        let from_toml = std::fs::read_to_string("./config.toml");
        from_toml
    }
}
