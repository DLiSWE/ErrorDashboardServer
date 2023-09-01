use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

use crate::config::Config;
use crate::shared::utils::errors::MyError;

pub fn create_pool() -> Result<r2d2::Pool<ConnectionManager<PgConnection>>, MyError> {
    let config = Config::from_env()?;
    let database_url = config.build_db_url();
    println!("{} dburl", database_url);
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    match r2d2::Pool::builder().build(manager) {
        Ok(pool) => Ok(pool),
        Err(err) => Err(MyError::PoolError(err))
    }
}
