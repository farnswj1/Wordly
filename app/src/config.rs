use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub cors_allowed_origins: String
}
