pub mod challenge;
pub mod health;
pub mod session;

use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health::health_check)
        .service(session::register_session)
        .service(challenge::add_challenge)
        .service(challenge::attempt_challenge);
}
