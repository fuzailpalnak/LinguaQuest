use actix_web::HttpResponse;
use actix_web::{error::ResponseError, App};
use log::error;
use serde::{Deserialize, Serialize};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("User not found")]
    UserNotFound,

    #[error("Challenge not found")]
    ChallengeNotFound,

    #[error("Question not found")]
    QuestionNotFound,

    #[error("Invalid attempt")]
    InvalidAttempt,

    #[error("Internal server error")]
    InternalServerError,
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AppError::UserNotFound => HttpResponse::NotFound().json(ErrorResponse {
                message: "User not found".to_string(),
            }),
            AppError::ChallengeNotFound => HttpResponse::NotFound().json(ErrorResponse {
                message: "Challenge not found".to_string(),
            }),
            AppError::QuestionNotFound => HttpResponse::NotFound().json(ErrorResponse {
                message: "Question not found".to_string(),
            }),
            AppError::InvalidAttempt => HttpResponse::BadRequest().json(ErrorResponse {
                message: "Invalid attempt".to_string(),
            }),
            AppError::InternalServerError => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    message: "Internal Server Error".to_string(),
                })
            }
        }
    }
}
