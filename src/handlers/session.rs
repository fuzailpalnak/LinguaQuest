use crate::state::AppState;
use crate::User;
use actix_web::{web, HttpResponse};
use log::info;

#[actix_web::post("/register_session")]
async fn register_session(
    state: web::Data<AppState>,
    user_registration: web::Json<User>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut users = state.users.lock().unwrap();
    users.push(user_registration.into_inner());
    info!("User registered successfully");
    Ok(HttpResponse::Ok().json("User Registered!"))
}
