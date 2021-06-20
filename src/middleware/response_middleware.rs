use actix_web::HttpResponse;
use serde::{Serialize};

use crate::middleware::errors_middleware::{CustomHttpError};

/// A generic HTTP responder. Different than actix-web's one.
/// This works better as it allows for a consistent JSON response layout across the entire API.
#[derive(Serialize)]
pub struct HttpResponseBuilder<T> {
    code: u16,
    message: T
}

impl<Body: Serialize> HttpResponseBuilder<Body> {
    /// Generates a new HTTP response builder.
    /// First, creates a new instance of the struct. 
    /// Then it matches the code to an HTTP response, and finally sends back the HttpResponseBuilder object.
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