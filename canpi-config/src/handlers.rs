use super::state::AppState;
use actix_web::{web, HttpResponse};

use super::models::Course;
use chrono::Utc;

// Configure handler
pub async fn config_display_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello.  canPiRust is alive")
}

