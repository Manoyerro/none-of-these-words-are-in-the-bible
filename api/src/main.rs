mod handlers;
mod models;
mod parser;

use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use std::env;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(handlers::config)
            .service(hello)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
