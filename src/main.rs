use crate::log::{Log};
use actix_web::{App, HttpServer};
use std::sync::Mutex;

mod log;
mod controllers;
mod dto;


pub struct AppState {
    log: Mutex<Log>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                log: Mutex::new(Log::new())
            })
            .service(controllers::health)
            .service(controllers::handle_produce)
            .service(controllers::handle_consume)
    }) .bind("0.0.0.0:8000")?.run().await

}
