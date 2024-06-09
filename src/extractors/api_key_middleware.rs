use std::rc::Rc;

use actix_web::{dev::{ServiceRequest, ServiceResponse}, HttpResponse};
use actix_web::dev::{forward_ready, Service, Transform};
use futures_util::future::{LocalBoxFuture, ok, Ready};
use futures_util::FutureExt;

pub struct ApiKeyMiddleware;

impl<S> Transform<S, ServiceRequest> for ApiKeyMiddleware
    where
        S: Service<ServiceRequest, Response = ServiceResponse<actix_web::body::BoxBody>, Error = actix_web::Error> + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Transform = ApiKeyMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ApiKeyMiddlewareService { service: Rc::new(service) })
    }
}

pub struct ApiKeyMiddlewareService<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for ApiKeyMiddlewareService<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<actix_web::body::BoxBody>, Error = actix_web::Error> + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, actix_web::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let api_key = std::env::var("API_KEY").expect("API_KEY must be set in .env file");

        if let Some(header_value) = req.headers().get("X-API-Key") {
            if header_value == api_key.as_str() {
                return self.service.call(req).boxed_local();
            }
        }

        let response = HttpResponse::Unauthorized().finish();
        let boxed_response = req.into_response(response.map_into_boxed_body());

        async move { Ok(boxed_response) }.boxed_local()
    }
}