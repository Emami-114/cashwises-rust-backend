use std::fmt;
use actix_web::{HttpResponse, ResponseError};
use crate::errors::auth_errors::ErrorMessage;
use crate::schema::response_schema::Response;

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: u16,
}
impl HttpError {
    pub fn new(message: impl Into<String>, status: u16) -> Self {
        HttpError {
            message: message.into(),
            status,
        }
    }
    pub fn server_error(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 500,
        }
    }
    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 400,
        }
    }
    pub fn unique_constraint_voilation(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 409,
        }
    }
    pub fn unauthorized(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 401,
        }
    }

    // pub fn not_found(message: impl Into<String>) -> Self {
    //     HttpError {
    //         message: message.into(),
    //         status: 404,
    //     }
    // }

    pub fn into_http_response(self) -> HttpResponse {
        match self.status {
            400 => HttpResponse::BadRequest().json(Response {
                status: "fail",
                message: self.message.into(),
            }),
            401 => HttpResponse::Unauthorized().json(Response {
                status: "fail",
                message: self.message.into(),
            }),
            409 => HttpResponse::Conflict().json(Response {
                status: "fail",
                message: self.message.into(),
            }),
            500 => HttpResponse::InternalServerError().json(Response {
                status: "error",
                message: self.message.into(),
            }),
            _ => {
                eprintln!(
                    "Warning: Missing pattern match. Converted status code {} to 500.",
                    self.status
                );

                HttpResponse::InternalServerError().json(Response {
                    status: "error",
                    message: ErrorMessage::ServerError.into(),
                })
            }
        }
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HttpError: message: {}, status: {}",
            self.message, self.status
        )
    }
}

impl std::error::Error for HttpError {}

impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let cloned = self.clone();
        cloned.into_http_response()
    }
}
