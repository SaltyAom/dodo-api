use actix_web::{ Responder, web, web::{ ServiceConfig } };

async fn status() -> impl Responder {
    "Working!"
}

pub fn status_module(config: &mut ServiceConfig) {
    config
        .route("/", web::get().to(status))
        .route("/status", web::get().to(status));
}