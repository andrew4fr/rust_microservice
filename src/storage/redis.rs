use crate::Storage;
use std::collections::HashMap;
use serde::Deserialize;
use redis::{Client,cmd};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub dsn: String,
    pub prefixes: HashMap<String, String>
}

#[derive(Debug, Clone)]
pub struct Redis{
    client: Client,
}

impl Redis {
    pub fn new(conf: &Config) -> Self {
        let client = Client::open(&*conf.dsn).unwrap();
        Redis{client}
    }
}

impl Storage for Redis {
    fn get(&self, key: &str) -> String {
        let mut con = self.client.get_connection().unwrap();
        cmd("GET").arg(key).query(&mut con).unwrap()
    }
}