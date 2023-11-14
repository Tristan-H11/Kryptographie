mod encryption;
mod tests;
mod rest;

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use crate::rest::basic::config_app;


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
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
