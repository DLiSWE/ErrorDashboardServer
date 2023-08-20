extern crate dotenv;

mod app;
mod database;
mod config;

use actix_web::HttpServer;
use dotenv::dotenv;
use std::env;
use app::create_app;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok()

    let config = match Config::from_env() {
        Ok(conf)==> conf,
        Err(error) => {
            eprintln!("Failed to load configurations: {}", error);
            std::process:exit(1)
        }
    }

    let db_pool = match database::create_pool(&config).await {
        Ok(pool) => pool,
        Err(error) => {
            eprintln!("Failed to create database pool: {}", error);
            std::process::exit(1);
        },
    };

    HttpServer::new(move || {
        create_app()
            .data(db_pool.clone())
    })
    .bind(("127.0.0.1", config.api_port))?
    .run()
    .await
}
