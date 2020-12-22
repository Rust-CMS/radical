use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};

use serde::{Serialize};
use thiserror::Error;
#[derive(Error, Debug)]
pub enum CustomHttpError {
    #[error("Incorrect parameter type.")]
    BadRequest,
    #[error("Resource not found.")]
    NotFound,
    #[error("Unknown Internal Error")]
    Unknown
}

impl CustomHttpError {
    pub fn name(&self) -> String {
        match self {
            Self::BadRequest => "Bad Request".to_string(),
            Self::Unknown => "Internal server error".to_string(),
            Self::NotFound => "Not Found".to_string()
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String
}

impl ResponseError for CustomHttpError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound => StatusCode::NOT_FOUND
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name()
        };

        HttpResponse::build(status_code).json(error_response)
    }
}

pub fn map_parsing_error(e: std::num::ParseIntError) -> CustomHttpError {
    match e.kind() {
        std::num::IntErrorKind::InvalidDigit => CustomHttpError::BadRequest,
        _ => CustomHttpError::Unknown
    }
}

pub fn map_sql_error(e: diesel::result::Error) -> CustomHttpError {
    match e {
        diesel::result::Error::NotFound => CustomHttpError::NotFound,
        _ => CustomHttpError::Unknown
    }
}