use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;

use crate::config::Config;
use crate::shared::utils::errors::MyError;

pub async fn create_pool(config: Arc<Config>) -> Result<DatabaseConnection, MyError> {
    let database_url = config.as_ref().build_db_url();

    match Database::connect(&database_url).await {
        Ok(database) => Ok(database),
        Err(err) => Err(MyError::DBError(err))
    }
}
