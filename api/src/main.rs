mod handlers;
mod models;
mod parser;

use actix_web::{middleware, App, HttpServer};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_default().parse().unwrap_or(8000);
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(handlers::config)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
