use deadpool_postgres::{Manager, Pool, RecyclingMethod};
use dotenv::dotenv;
use std::env;
use tokio_postgres::NoTls;

pub async fn create_pool() -> Pool {
    dotenv().ok(); // Load .env file if available

    // Get the DATABASE_URL environment variable
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the environment or .env file");

    // Create a configuration object for the pool
    let cfg = deadpool_postgres::Config {
        user: Some(env::var("DB_USER").expect("DB_USER must be set")),
        password: Some(env::var("DB_PASS").expect("DB_PASS must be set")),
        dbname: Some(env::var("DB_NAME").expect("DB_NAME must be set")),
        host: Some(env::var("DB_HOST").expect("DB_HOST must be set")),
        port: Some(env::var("DB_PORT").unwrap_or_else(|_| "5434".to_string()).parse().unwrap()),
        ..Default::default()
    };

    let pool = cfg.create_pool(NoTls).expect("Failed to create database connection pool");
    
    pool
}
