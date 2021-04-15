#[macro_use]
extern crate lazy_static;
extern crate juniper;

mod models;
mod modules;
mod services;

use std::sync::Arc;

use actix_web::{ App, HttpServer, middleware::Compress, http };
use actix_cors::Cors;

use modules::{status::controllers::status_module, graphql::controllers::graphql_module};
use services::graphql::graphql::create_schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Arc::new(create_schema());

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
            .max_age(86400);

        App::new()
            .wrap(cors)
            .wrap(Compress::default())
            .data(schema.clone())
            .configure(status_module)
            .configure(graphql_module)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
