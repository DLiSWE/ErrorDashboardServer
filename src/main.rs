mod database;
mod config;
mod handlers;
mod models;
mod routes;
mod services;
mod dtos;
mod shared {
    pub mod utils;
}

use actix_web::{middleware, web, App, HttpServer};
use log::{ error, info };

use crate::routes::user_routes;
use config::Config;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = match Config::from_env() {
        Ok(conf) => {
            env_logger::init();
            info!("Successfully loaded configurations.");
            conf},
        Err(error) => {
            env_logger::init();
            error!("Failed to load configurations: {}", error);
            std::process::exit(1)
        }
    };

    let db_pool = match database::create_pool().await {
        Ok(pool) =>{
            info!("Successfully connected to database.");
            pool},
        Err(error) => {
            error!("Failed to create database pool: {}", error);
            std::process::exit(1);
        },
    };

    println!("+----------------------------------------------------------------+");
    println!("|                                                                ");
    println!("|    Initializing Server                                         ");
    println!("|    Listening on {}:{}...                              ", config.db_host, config.api_port);
    println!("|                                                                ");
    println!("+----------------------------------------------------------------+"); 

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
