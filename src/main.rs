mod handlers;
mod models;
mod response;
mod state;

use actix_web::{web, App, HttpServer};
use models::{Attempt, Challenge, Response, User};
use response::AppError;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let app_state = state::AppState::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .configure(handlers::config_routes)
    })
    .bind("127.0.0.1:8080")?
    .workers(2)
    .run()
    .await
}
