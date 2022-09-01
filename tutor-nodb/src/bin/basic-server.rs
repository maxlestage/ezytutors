// Module imports
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// Configure Route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello. Eztutors is alive and kicking!")
}

pub fn custom_route(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(custom_check_handler));
}

pub async fn custom_check_handler() -> impl Responder {
    HttpResponse::Ok().json("You're on a custom routing path !")
}

// Instantiate and run the HTTP server
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Construct app and configure routes
    let app = move || App::new().configure(general_routes);

    // Start HTTP server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
