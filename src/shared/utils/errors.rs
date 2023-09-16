use std::fmt;
use std::error::Error;
use anyhow::Error as AnyhowError;
use sea_orm::error::{DbErr, SqlErr};
use actix_web::Error as ActixError;

#[derive(Debug)]
pub enum MyError {
    PoolError(SqlErr),
    DBError(DbErr),
    WebError(ActixError),
    AnyhowError(AnyhowError)
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::PoolError(err) => write!(f, "PoolError: {}", err),
            MyError::DBError(err) => write!(f, "DBError: {}", err),
            MyError::WebError(err) => write!(f, "WebError: {}", err),
            MyError::AnyhowError(err ) => write!(f, "AnyhowError: {}", err)
        }
    }
}

impl Error for MyError {}

impl From<SqlErr> for MyError {
    fn from(err: SqlErr) -> MyError {
        MyError::PoolError(err)
    }
}

impl From<DbErr> for MyError {
    fn from(err: DbErr) -> MyError {
        MyError::DBError(err)
    }
}

impl From<ActixError> for MyError {
    fn from(err: ActixError) -> MyError {
        MyError::WebError(err)
    }
}

impl From<AnyhowError> for MyError {
    fn from(err: AnyhowError) -> MyError {
        MyError::AnyhowError(err)
    }
}
