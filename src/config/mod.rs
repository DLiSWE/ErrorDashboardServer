use anyhow::{Result, Context};
use std::env;

pub struct Config {
    pub environment: String,
    pub secret_key: String,
    pub hash_cost: String,
    pub api_port: u16,
    pub db_user: String,
    pub db_pass: String,
    pub db_name: String,
    pub db_host: String,
    pub db_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Config> {
        dotenv::from_filename(".env").ok();

        let environment = env::var("ENVIRONMENT")
            .context("ENVIRONMENT must be set in the environment or .env file")?;

        let env_file = format!(".env.{}.local", environment);
        dotenv::from_filename(&env_file).ok();
        
        let secret_key = env::var("SECRET_KEY")
            .context("SECRET_KEY must be set in the environment or .env file")?;

        let hash_cost = env::var("HASH_COST")
            .context("HASH_COST must be set in the environment or .env file")?;

        let api_port: u16 = env::var("API_PORT")
            .context("API_PORT must be set in the environment or .env file")?
            .parse()
            .context("API_PORT must be a valid number")?;

        let db_user = env::var("DB_USER")
            .context("DB_USER must be set in the environment or .env file")?;

        let db_pass = env::var("DB_PASS")
            .context("DB_PASS must be set in the environment or .env file")?;

        let db_name = env::var("DB_NAME")
            .context("DB_NAME must be set in the environment or .env file")?;

        let db_host = env::var("DB_HOST")
            .context("DB_HOST must be set in the environment or .env file")?;

        let db_port: u16 = env::var("DB_PORT")
            .context("DB_PORT must be set in the environment or .env file")?
            .parse()
            .context("DB_PORT must be a valid number")?;

        Ok(Config {
            environment,
            secret_key,
            hash_cost,
            api_port,
            db_user,
            db_pass,
            db_name,
            db_host,
            db_port,
        })
    }

    pub fn build_db_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.db_user, self.db_pass, self.db_host, self.db_port, self.db_name
        )
    }
}

