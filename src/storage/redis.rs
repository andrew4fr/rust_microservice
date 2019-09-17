use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub dsn: String,
    pub prefixes: HashMap<String, String>
}

pub struct Redis;

impl Redis {
    pub fn new (_conf: &Config) -> Self {
        Redis{}
    }
}