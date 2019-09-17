use actix_web::HttpRequest;

pub fn common_dicts(_req: HttpRequest) -> &'static str {
    "Common dicts"
}

pub fn client_dicts(_req: HttpRequest) -> &'static str {
    "Client dicts"
}