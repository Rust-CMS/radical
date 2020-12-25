use actix_web::HttpResponse;
use serde::{Serialize};

use crate::errors_middleware::{CustomHttpError};

#[derive(Serialize)]
pub struct HttpResponseBuilder<T> {
    code: u16,
    message: T
}

impl<Body: Serialize> HttpResponseBuilder<Body> {
    pub fn new(code: u16, message: &Body) -> Result<HttpResponse, CustomHttpError> {
        let cm = HttpResponseBuilder {
            code,
            message
        };

        // Eventually, the final arm of this match needs to be a CustomHttpError.
        let mut m = match code {
            200 => HttpResponse::Ok(),
            201 => HttpResponse::Created(),
            204 => HttpResponse::NoContent(),
            _ => HttpResponse::BadRequest()
        };

        Ok(m.body(serde_json::to_string(&cm).or(Err(CustomHttpError::Unknown))?))
    }
}