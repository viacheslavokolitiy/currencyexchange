use std::future::{ready, Ready};
use std::pin::Pin;
use std::rc::Rc;
use actix_web::body::BoxBody;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{error, Error, HttpMessage, HttpRequest, HttpResponse};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use jsonwebtoken::errors::ErrorKind;
use crate::utils::env_parser::EnvParser;
use crate::utils::jwt::Claims;

pub struct JwtMiddleware;
pub struct JwtMiddlewareService<S> {
    service: Rc<S>,
}

impl<S> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>
    + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = JwtMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

impl<S> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>
    + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let parser = EnvParser::new();
        let srv = self.service.clone();
        Box::pin(async move {
            let auth_header = req.headers().get("Authorization");
            if auth_header.is_none() {
                return Err(error::ErrorUnauthorized("Missing Authorization header"));
            }

            let auth_str = match auth_header.unwrap().to_str() {
                Ok(s) => s,
                Err(_) => return Err(error::ErrorBadRequest("Invalid Authorization header")),
            };

            if !auth_str.starts_with("Bearer ") {
                return Err(error::ErrorBadRequest("Invalid Authorization format"));
            }

            let token = auth_str[7..].trim();

            let mut validation = Validation::new(Algorithm::HS256);
            validation.set_required_spec_claims(&["exp", "iat", "sub"]);
            let token_data = match decode::<Claims>(
                token,
                &DecodingKey::from_secret(parser.jwt_secret().as_ref()),
                &validation,
            ) {
                Ok(data) => data,
                Err(err) => {
                    let msg = match err.kind() {
                        ErrorKind::ExpiredSignature => "Token expired",
                        ErrorKind::InvalidToken => "Invalid token",
                        _ => "Token validation failed",
                    };
                    return Err(error::ErrorUnauthorized(msg));
                }
            };

            req.extensions_mut().insert(token_data.claims);

            srv.call(req).await
        })
    }
}

pub async fn protected(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("You are authenticated! This is protected content.")
}