mod database;
mod dtos;
mod config;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod services;
mod shared {
    pub mod utils;
}

use actix_web::{middleware, web, App, HttpServer};
use log::{ error, info };
use std::sync::Arc;

use crate::routes::user_routes;
use crate::routes::auth_routes;
use config::Config;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = match Config::from_env() {
        Ok(conf) => {
            env_logger::init();
            info!("Successfully loaded configurations.");
            Arc::new(conf)},
        Err(error) => {
            env_logger::init();
            error!("Failed to load configurations: {}", error);
            std::process::exit(1)
        }
    };

    let config_for_bind = Arc::clone(&config);

    let db_pool = match database::create_pool().await {
        Ok(pool) =>{
            info!("Successfully connected to database.");
            Arc::new(pool)},
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
            .app_data(web::Data::new(Arc::clone(&config)))
                .wrap(middleware::Logger::default())
                    .configure(user_routes::configure)
                    .configure(auth_routes::configure)
    })
    .bind(("127.0.0.1", config_for_bind.api_port))?
    .run()
    .await
}
