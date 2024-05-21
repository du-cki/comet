use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
    pub bind_addr: [u8; 4],
    pub bind_port: u16,
    pub password: String,
    pub file_name_length: usize,
    pub enforce_file_extensions: bool,
    pub retain_uploaded_file_name: bool, // TODO
    pub file_save_path: String,
    pub default_public: bool,
    pub fallback_content_type: String,
    pub file_size_limit: usize,
    pub dashboard: Dashboard,
    pub api_endpoints: ApiEndpoints,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Dashboard {
    pub enabled: bool,
    pub base_path: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ApiEndpoints {
    pub get: String,
    pub upload: String,
    pub delete: String,
    pub ping: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(
                File::with_name("comet-config.toml")
            )
            .add_source(
                Environment::default()
            )
            .build()?
            .try_deserialize()
    }
}
