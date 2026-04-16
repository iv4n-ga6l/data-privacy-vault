use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage, HttpResponse};
use actix_service::{Service, Transform};
use futures::future::{ok, Ready};
use std::collections::HashMap;
use std::future::{ready, Future};
use std::pin::Pin;
use std::task::{Context, Poll};
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    static ref API_KEYS: RwLock<HashMap<String, String>> = {
        let mut keys = HashMap::new();
        keys.insert("writer-key-123", "writer".to_string());
        keys.insert("reader-key-456", "reader".to_string());
        RwLock::new(keys)
    };
}

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService { service })
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let api_key = req.headers().get("x-api-key").and_then(|v| v.to_str().ok());
        let role = api_key.and_then(|key| API_KEYS.read().unwrap().get(key).cloned());

        let is_authorized = match (req.path(), role.as_deref()) {
            ("/tokenize", Some("writer")) => true,
            ("/detokenize", Some("reader")) => true,
            _ => false,
        };

        if is_authorized {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async move {
                Ok(req.into_response(HttpResponse::Unauthorized().finish().into_body()))
            })
        }
    }
}
