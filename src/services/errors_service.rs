use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use thiserror::Error;

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
    pub fn descriptor(&self) -> String {
        match self {
            Self::BadRequest => String::from("Server was unable to handle data"),
            Self::Unknown => String::from("Internal server error"),
            Self::NotFound => String::from("Resource was not found"),
        }
    }
}

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
            message: self.descriptor(),
            error: self.to_string(),
        };

        HttpResponse::build(status_code).json(error_response)
    }
}

/// Any time an SQL query fails, it gets mapped to here.
impl From<diesel::result::Error> for CustomHttpError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => CustomHttpError::NotFound,
            _ => CustomHttpError::Unknown,
        }
    }
}

impl From<jsonwebtoken::errors::Error> for CustomHttpError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        match e {
            _ => CustomHttpError::Unknown
        }
    }
}