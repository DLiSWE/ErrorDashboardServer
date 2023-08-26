use deadpool_postgres::{Config, Pool};
use anyhow::Result;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use std::env;

pub async fn create_pool() -> Result<Pool> {
    dotenv().ok();
   
    let mut cfg = Config::default();
    cfg.user = Some(env::var("DB_USER")?);
    cfg.password = Some(env::var("DB_PASS")?);
    cfg.dbname = Some(env::var("DB_NAME")?);
    cfg.host = Some(env::var("DB_HOST")?);
    cfg.port = Some(env::var("DB_PORT").unwrap_or_else(|_| "5434".to_string()).parse()?);

    let pool = cfg.create_pool(None, NoTls)?;
    
    Ok(pool)
}
