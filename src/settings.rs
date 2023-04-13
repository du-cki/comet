use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
    pub bind_addr: [u8; 4],
    pub bind_port: u16,
    pub password: String,
    pub file_name_length: usize,
    pub enforce_file_extensions: bool,
    pub file_save_path: String,
    pub fallback_content_type: String,
    pub endpoints: Endpoints,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Endpoints {
    pub get_file: String,
    pub upload_file: String,
    pub delete_file: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::with_name("comet-config.toml"))
            .add_source(Environment::default())
            .build()?
            .try_deserialize()
    }
}
