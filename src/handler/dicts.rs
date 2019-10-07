use actix_web::{HttpRequest, Responder, HttpResponse};
use crate::AppState;
use crate::storage::Storage;
use slog::info;

pub fn common_dicts(req: HttpRequest) -> impl Responder {
    let data = req.app_data::<AppState<dyn Storage>>().unwrap();
    let log = &data.log;
    let storage = &data.storage;

    info!(log, "get common dicts");

    let res = storage.get("common_dicts:packages");

    HttpResponse::Ok()
        .content_type("application/json")
        .body(res)
}

pub fn client_dicts(_req: HttpRequest) -> impl Responder {
    "Client dicts"
}