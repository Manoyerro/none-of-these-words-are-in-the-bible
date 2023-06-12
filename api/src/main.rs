mod parser;

use actix_web::{
    delete, get, post, put, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
    ResponseError,
};
use std::env;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get $PORT or default to 8000
    let port = env::var("PORT")
        .unwrap_or("8000".parse().unwrap())
        .parse::<u16>()
        .unwrap();
    HttpServer::new(move || App::new().service(hello))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
