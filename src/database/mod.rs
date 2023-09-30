use sea_orm::{Database, DatabaseConnection};

use crate::config::Config;
use crate::shared::utils::errors::MyError;

pub async fn create_pool() -> Result<DatabaseConnection, MyError> {
    let config = Config::from_env()?;
    let database_url = config.build_db_url();


    match Database::connect(&database_url).await {
        Ok(database) => Ok(database),
        Err(err) => Err(MyError::DBError(err))
    }
}
