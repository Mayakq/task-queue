pub mod config {
    use core::panic;

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
        pub fn new() -> Self {
            init_config()
        }
    }

    pub fn init_config() -> Config {
        let toml = read_toml();
        return match toml {
            Ok(str) => {
                let config: Result<Config, toml::de::Error> = toml::from_str(str.as_str());
                match config {
                    Ok(config) => {
                        info!("init - init_config | Parse Toml config - {:#?}", config);
                        config
                    }
                    Err(err) => {
                        error!("init - init_config | {}", err);
                        panic!("{}", format!("init - init_config | {}", err));
                    }
                }
            }
            Err(error) => {
                let error = error.to_string();
                tracing::error!(error);
                panic!("{}", format!("init - init_config | {}", error));
            }
        };
    }
    fn read_toml() -> std::io::Result<String> {
        let from_toml = std::fs::read_to_string("./config.toml");
        from_toml
    }
}
