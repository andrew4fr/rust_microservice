use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Mutex;
use slog::o;
use slog::Drain;

mod error;
mod metrix;
mod middleware;
mod handler;
mod storage;

use error::ConfigError;
use storage::{Storage, Redis};

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
struct AppState<S: Storage> {
    system_token: String,
    log: slog::Logger,
    storage: S,
}

fn main() {
    let config = read_config().unwrap();

    let log = setup_logging();
    let storage = Redis::new(&config.storage);

    let state = AppState {
        system_token: config.system_token.to_owned(),
        log: log.clone(),
        storage:  storage,
    };

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
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
                        "/dicts/{dicts}",
                        web::get().to(handler::common_dicts)
                    )
                    .route(
                        "/account/{account}/dicts/{dicts}",
                        web::get().to(handler::client_dicts)
                    )
            )
    })
    .bind(format!("127.0.0.1:{}", config.port))
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

fn setup_logging() -> slog::Logger {
    let mut builder = slog_json::Json::new(std::io::stdout());
    builder = builder
        .add_key_value(o!("service" => "target-dicts"))
        .add_default_keys()
    ;

    let drain = Mutex::new(builder.build()).map(slog::Fuse);
    slog::Logger::root(drain, o!())
}