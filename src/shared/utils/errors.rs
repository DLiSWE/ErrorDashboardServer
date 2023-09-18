use std::fmt;
use std::error::Error;
use anyhow::Error as AnyhowError;
use sea_orm::error::{DbErr, SqlErr};
use actix_web::{Error as ActixError, ResponseError as ActixResponseError, HttpResponse};
use actix_web::http::StatusCode;
use bcrypt::BcryptError;

#[derive(Debug)]
pub struct HttpError {
    pub status: StatusCode,
    pub message: String,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HttpError: {} - {}", self.status, self.message)
    }
}

impl Error for HttpError {}

impl From<HttpError> for HttpResponse {
    fn from(err: HttpError) -> HttpResponse {
        HttpResponse::build(err.status).json(err.message)
    }
}

#[derive(Debug)]
pub enum MyError {
    PoolError(SqlErr),
    DBError(DbErr),
    WebError(HttpError),
    AnyhowError(AnyhowError),
    ActixError(ActixError),
    BcryptError(BcryptError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::PoolError(err) => write!(f, "PoolError: {}", err),
            MyError::DBError(err) => write!(f, "DBError: {}", err),
            MyError::WebError(err) => write!(f, "WebError: {}", err),
            MyError::AnyhowError(err) => write!(f, "AnyhowError: {}", err),
            MyError::ActixError(err) => write!(f, "ActixError: {}", err),
            MyError::BcryptError(err) => write!(f, "BcryptError: {}", err),
        }
    }
}

impl Error for MyError {}

impl ActixResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match self {
            MyError::WebError(http_err) => HttpResponse::build(http_err.status).json(http_err.message.clone()),
            MyError::PoolError(_) | MyError::DBError(_) | MyError::AnyhowError(_) | MyError::BcryptError(_) => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            },
            MyError::ActixError(_) => HttpResponse::InternalServerError().json("Actix Internal Error"),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            MyError::WebError(http_err) => http_err.status,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

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

impl From<AnyhowError> for MyError {
    fn from(err: AnyhowError) -> MyError {
        MyError::AnyhowError(err)
    }
}

impl From<ActixError> for MyError {
    fn from(err: ActixError) -> MyError {
        MyError::ActixError(err)
    }
}

impl From<BcryptError> for MyError {
    fn from(err: BcryptError) -> MyError {
        MyError::BcryptError(err)
    }
}

impl From<HttpError> for MyError {
    fn from(err: HttpError) -> MyError {
        MyError::WebError(err)
    }
}
