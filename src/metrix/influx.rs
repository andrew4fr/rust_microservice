use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub dsn: String,
    pub measurement: String,
}