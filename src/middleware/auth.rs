use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::http::Method;
use actix_web::HttpResponse;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, Either, Ready};

use crate::models::MySQLPool;
use crate::services::auth_service::{compare, decrypt};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Authorization;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for Authorization
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthorizationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthorizationMiddleware { service })
    }
}

pub struct AuthorizationMiddleware<S> {
    service: S,
}

const RESTRICTED_METHODS: [Method; 3] = [Method::POST, Method::PUT, Method::DELETE];

impl<S, B> Service for AuthorizationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        match authenticate(&req) {
            Ok(_) => Either::Left(self.service.call(req)),
            Err(_) => Either::Right(ok(
                req.into_response(HttpResponse::Unauthorized().finish().into_body())
            )),
        }
    }
}

fn authenticate(req: &ServiceRequest) -> Result<(), ()> {
    if !RESTRICTED_METHODS.contains(req.method()) {
        return Ok(());
    }

    let auth_header = req.headers().get("Authorization").ok_or(())?;

    let encrypted_token = std::str::from_utf8(auth_header.as_bytes())
        .unwrap()
        .to_string();

    let decrypted_token = decrypt(&encrypted_token).ok_or(())?;

    let is_logged_in = compare(
        decrypted_token,
        &encrypted_token,
        req.app_data::<MySQLPool>().unwrap().to_owned(),
    );

    if is_logged_in {
        Ok(())
    } else {
        Err(())
    }
}
