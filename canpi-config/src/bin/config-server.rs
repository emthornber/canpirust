// Module imports
use actix_web::{web, App, HttpServer};
use std::io;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;

use routes::*;
use state::AppState;

// Instantiate and run the HTTP server
#[actix_rt::main]
async fn main() -> io::Result<()> {
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });
 
    // Construct all and configure routes
    let app = move || App::new().configure(general_routes);

    // Start HTTP server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
