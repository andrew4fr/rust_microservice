use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, error};
use futures::future::{ok, err, FutureResult};
use futures::{Future, Poll};
use crate::AppState;

pub struct CheckSystemToken;

impl<S, B> Transform<S> for CheckSystemToken
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckSystemTokenMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckSystemTokenMiddleware { service })
    }
}

pub struct CheckSystemTokenMiddleware<S> {
    service: S,
}

impl<S, B> Service for CheckSystemTokenMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let token: String = match req.headers().get("X-System-Token") {
            Some(x) => {
                match x.to_str() {
                    Ok(y) => y.into(),
                    Err(_) => "".into(),
                }
            },
            None => "".into()
        };

        let mut sys_token = String::new();
        if let Some(data) = req.app_data::<AppState>() {
            sys_token = data.system_token.clone();
        }

        if token != sys_token {
            Box::new(err(error::ErrorForbidden("Forbidden")))
        } else {
            Box::new(self.service.call(req).and_then(move |res| {
                ok(res)
            }))
        }
    }
}