use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

mod error;
mod metrix;
mod middleware;
mod handler;
mod storage;

use error::ConfigError;
//use storage::Redis;

const HEALTH_ANSWER: &str = r#"{"error": null, "result": "ok"}"#;

#[derive(Deserialize, Debug, Clone)]
struct Config {
    service_name: String,
    host_name: String,
    port: String,
    system_token: String,
    debug: bool,
    target_api: String,
    services: HashMap<String, String>,
    storage: storage::Config,
    dicts: Dicts,
    metrix: metrix::Config,
}

#[derive(Deserialize, Debug, Clone)]
struct Dicts {
    client: HashMap<String, DictParams>,
    common: HashMap<String, DictParams>,
    fields: HashMap<String, String>,
}

#[derive(Deserialize, Debug, Clone)]
struct DictParams {
    #[serde(default)]
    url: String,
    #[serde(default)]
    root: String,
    limit: Option<u8>,
}

#[derive(Clone,Debug)]
struct AppState {
    system_token: String,
}

fn main() {
    let config = read_config().unwrap();
    let port = config.port.to_string();

    let data = AppState {
        system_token: config.system_token.to_string(),
    };


//    let storage = Redis::new(&config.storage);

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .route(
                "health",
                web::get().to(|| HttpResponse::Ok().body(HEALTH_ANSWER)),
            )
            .service(
                web::scope("/")
                    .wrap(middleware::CheckRequestID)
                    .wrap(middleware::CheckSystemToken)
                    .route(
                        "hello",
                        web::get().to(|| HttpResponse::Ok().body("Hello\n")),
                    )
                    .route(
                        "commondicts",
                        web::to(handler::common_dicts)
                    )
                    .route(
                        "clientdicts",
                        web::to(handler::client_dicts)
                    )
            )
    })
    .bind(format!("127.0.0.1:{}", port))
    .unwrap()
    .run()
    .unwrap();
}

fn read_config() -> Result<Config, ConfigError> {
    let mut file = File::open("config/config.toml")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let config = toml::from_str::<Config>(&buffer)?;

    Ok(config)
}
