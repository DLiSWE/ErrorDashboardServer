extern crate dotenv;

mod database;
mod config;
mod handlers;
mod models;
mod routes;
mod services;

use actix_web::{middleware, web, App, HttpServer};
use crate::routes::user_routes;
use config::Config;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();


    let config = match Config::from_env() {
        Ok(conf) => conf,
        Err(error) => {
            eprintln!("Failed to load configurations: {}", error);
            std::process::exit(1)
        }
    };

    let db_pool = match database::create_pool().await {
        Ok(pool) => pool,
        Err(error) => {
            eprintln!("Failed to create database pool: {}", error);
            std::process::exit(1);
        },
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
                .wrap(middleware::Logger::default())
                    .configure(user_routes::configure)
    })
    .bind(("127.0.0.1", config.api_port))?
    .run()
    .await
}
