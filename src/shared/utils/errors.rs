use std::fmt;
use std::error::Error;
use r2d2::Error as R2D2Error;
use anyhow::Error as AnyhowError;
use diesel::result::Error as DieselError;
use actix_web::Error as ActixError;

#[derive(Debug)]
pub enum MyError {
    PoolError(R2D2Error),
    DBError(DieselError),
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

impl From<R2D2Error> for MyError {
    fn from(err: R2D2Error) -> MyError {
        MyError::PoolError(err)
    }
}

impl From<DieselError> for MyError {
    fn from(err: DieselError) -> MyError {
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
