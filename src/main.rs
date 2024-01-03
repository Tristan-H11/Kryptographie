mod api;
mod encryption;
mod shared;

use crate::api::basic::config_app;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use log::LevelFilter;
use simple_logger::SimpleLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_colors(true)
        .init()
        .unwrap();

    HttpServer::new(|| {
        App::new()
            .configure(config_app)
            .wrap(Logger::default())
            .wrap(Cors::permissive())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
