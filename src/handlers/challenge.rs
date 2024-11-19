use crate::state::AppState;
use crate::AppError;
use crate::Response;
use crate::{Attempt, Challenge};
use actix_web::{web, HttpResponse};
use log::{error, info};
use std::sync::PoisonError;

#[actix_web::post("/challenge")]
async fn add_challenge(
    state: web::Data<AppState>,
    challenge: web::Json<Challenge>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut challenges = state
        .challenges
        .lock()
        .map_err(|e: PoisonError<_>| AppError::InternalServerError)?;

    let mut meta = state
        .meta
        .lock()
        .map_err(|e: PoisonError<_>| AppError::InternalServerError)?;

    let extracted_challenge = challenge.into_inner();
    info!("Adding challenge: {:?}", extracted_challenge);
    meta.insert(
        extracted_challenge.id,
        extracted_challenge.description.clone(),
    );
    challenges.insert(extracted_challenge.id, extracted_challenge);
    Ok(HttpResponse::Ok().json("Challenge added!"))
}

#[actix_web::post("/attempt")]
async fn attempt_challenge(
    state: web::Data<AppState>,
    attempt: web::Json<Attempt>,
) -> Result<HttpResponse, actix_web::Error> {
    let challenges = state
        .challenges
        .lock()
        .map_err(|e: PoisonError<_>| AppError::InternalServerError)?;

    if let Some(challenge) = challenges.get(&attempt.challenge_id) {
        if let Some(question) = challenge
            .questions
            .iter()
            .find(|q| q.id == attempt.question_id)
        {
            if attempt.user_option >= question.options.len() as u8 {
                error!("Invalid user option: {}", attempt.user_option);
                return Err(AppError::InvalidAttempt.into());
            }

            let response = if question.correct_option == attempt.user_option {
                Response {
                    point: question.point,
                }
            } else {
                Response { point: 0 }
            };

            info!("Attempt result: {:?}", response);
            return Ok(HttpResponse::Ok().json(response));
        } else {
            error!("Question not found for challenge");
            return Err(AppError::QuestionNotFound.into());
        }
    }

    error!("Challenge not found for attempt");
    Err(AppError::ChallengeNotFound.into())
}
