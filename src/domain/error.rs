use std::fmt::Display;

use actix_web::ResponseError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CommonError {
    pub message: String,
    pub code: u32
}

impl Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error {}: {}", self.code, self.message)
    }
}

impl ResponseError for CommonError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self.code {
            400 => actix_web::http::StatusCode::BAD_REQUEST,
            401 => actix_web::http::StatusCode::UNAUTHORIZED,
            403 => actix_web::http::StatusCode::UNAUTHORIZED,
            404 => actix_web::http::StatusCode::NOT_FOUND,
            500 => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code())
            .json(self)
    }
}
