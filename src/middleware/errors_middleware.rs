use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};

use serde::Serialize;
use thiserror::Error;
/// A custom type that defines errors that are mapped to HTTP errors later.
#[derive(Error, Debug)]
pub enum CustomHttpError {
    #[error("Incorrect parameter type.")]
    BadRequest,
    #[error("Resource not found.")]
    NotFound,
    #[error("Unknown Internal Error")]
    Unknown,
}

/// Provides an interface for getting a description of the request.
impl CustomHttpError {
    pub fn name(&self) -> String {
        match self {
            Self::BadRequest => String::from("Bad Request"),
            Self::Unknown => String::from("Internal server error"),
            Self::NotFound => String::from("Not Found"),
        }
    }
}
/// Struct that gets serialized and sent back to the user.
#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

/// Full implementation of ResponseError trait so that it can be sent back as an error through actix-web.
impl ResponseError for CustomHttpError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name(),
        };

        HttpResponse::build(status_code).json(error_response)
    }
}

/// Used whenever parsing ints out of the URLs.
pub fn map_int_parsing_error(e: std::num::ParseIntError) -> CustomHttpError {
    match e.kind() {
        std::num::IntErrorKind::InvalidDigit => CustomHttpError::BadRequest,
        _ => CustomHttpError::Unknown,
    }
}

/// Any time an SQL query fails, it gets mapped to here.
pub fn map_sql_error(e: diesel::result::Error) -> CustomHttpError {
    match e {
        diesel::result::Error::NotFound => CustomHttpError::NotFound,
        _ => CustomHttpError::Unknown,
    }
}