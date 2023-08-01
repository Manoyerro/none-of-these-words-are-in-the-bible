mod handlers;
mod models;
mod parser;

use actix_web::{middleware, App, HttpServer};
use std::env;
use std::net::{Ipv4Addr, SocketAddrV4};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get port from $PORT, defaulting to 8000
    let port = env::var("PORT").unwrap_or_default().parse().unwrap_or(8000);
    // Get address, hosting on 0.0.0.0 when PORT is set (assuming production), and defaulting to 127.0.0.1
    let address = if env::var("PORT").is_ok() {
        SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port)
    } else {
        SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port)
    };

    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(handlers::config)
    })
    .bind(address)?
    .run()
    .await
}
