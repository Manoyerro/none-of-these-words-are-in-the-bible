mod handlers;
mod models;
mod parser;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().configure(handlers::config).service(hello))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
