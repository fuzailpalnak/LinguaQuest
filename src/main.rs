mod models;
mod response;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::{error, info};
use models::{Attempt, Challenge, Response, User};
use response::AppError;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, PoisonError},
};
use uuid::Uuid;
type SharedData<T> = Arc<Mutex<Vec<T>>>;
type SharedMap<T, V> = Arc<Mutex<HashMap<T, V>>>;

#[derive(Clone)]
struct AppState {
    users: SharedData<User>,
    challenges: SharedMap<Uuid, Challenge>,
    meta: SharedMap<Uuid, String>,
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json("Server is running")
}

async fn register_session(
    state: web::Data<AppState>,
    user_registration: web::Json<User>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut users = state.users.lock().unwrap();
    users.push(user_registration.into_inner());
    info!("User registered successfully");
    Ok(HttpResponse::Ok().json("User Registered!"))
}

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let app_state = AppState {
        users: Arc::new(Mutex::new(Vec::new())),
        challenges: Arc::new(Mutex::new(HashMap::new())),
        meta: Arc::new(Mutex::new(HashMap::new())),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/", web::get().to(health_check))
            .route("/register", web::post().to(register_session))
            .route("/add_challenge", web::post().to(add_challenge))
            .route("/attempt", web::post().to(attempt_challenge))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
