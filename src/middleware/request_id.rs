use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use actix_web::http::header::{HeaderName, HeaderValue};
use futures::future::{ok, FutureResult};
use futures::{Future, Poll};
use uuid::Uuid;

pub struct CheckRequestID;

impl<S, B> Transform<S> for CheckRequestID
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckRequestIDMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckRequestIDMiddleware { service })
    }
}

pub struct CheckRequestIDMiddleware<S> {
    service: S,
}

impl<S, B> Service for CheckRequestIDMiddleware<S>
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
        let request_id = match req.headers().get("X-RequestID") {
            None => gen_uuid(),
            Some(u) => u.to_owned(),
        };

        Box::new(self.service.call(req).and_then(|mut res| {
            res.headers_mut().insert(HeaderName::from_static("x-requestid"), request_id);
            Ok(res)
        }))
    }
}

fn gen_uuid() -> HeaderValue {
    HeaderValue::from_bytes(Uuid::new_v4().to_string().as_bytes()).unwrap()
}