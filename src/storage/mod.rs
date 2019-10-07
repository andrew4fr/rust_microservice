mod redis;

pub trait Storage {
    fn get(&self, key: &str) -> String;
}

pub use self::redis::{Config, Redis};
