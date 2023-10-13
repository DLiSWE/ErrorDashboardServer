use std::fmt::{Formatter, Result, Display};
use std::error::Error;
use anyhow::Error as AnyhowError;
use sea_orm::error::{DbErr, SqlErr};
use serde_json::Error as JsonError;
use actix_web::{Error as ActixError, ResponseError as ActixResponseError, HttpResponse};
use actix_web::http::StatusCode;
use bcrypt::BcryptError;
use jsonwebtoken::errors::Error as JwtError;
use uuid::Error as UuidError;

#[derive(Debug)]
pub struct HttpError {
    pub status: StatusCode,
    pub message: String,
}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter) -> Result {
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
    // 3rd party errors
    ActixError(ActixError),
    AnyhowError(AnyhowError),
    BcryptError(BcryptError),
    DBError(DbErr),
    JsonError(JsonError),
    JwtError(JwtError),
    PoolError(SqlErr),
    UuidError(UuidError),
    WebError(HttpError),

    // Query errors
    UserNotFound,

    // Request errors
    InvalidHeader,
    InvalidToken,
    MissingHeader,
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            // 3rd party errors
            MyError::ActixError(err) => write!(f, "ActixError: {}", err),
            MyError::AnyhowError(err) => write!(f, "AnyhowError: {}", err),
            MyError::BcryptError(err) => write!(f, "BcryptError: {}", err),
            MyError::DBError(err) => write!(f, "DBError: {}", err),
            MyError::JsonError(err) => write!(f, "JsonError: {}", err),
            MyError::JwtError(err) => write!(f, "JwtError: {}", err),
            MyError::PoolError(err) => write!(f, "PoolError: {}", err),
            MyError::UuidError(err) => write!(f, "UuidError: {}", err),
            MyError::WebError(err) => write!(f, "WebError: {}", err),

            // Query errors
            MyError::UserNotFound => write!(f, "User not found"),

            // Request errors
            MyError::InvalidHeader => write!(f, "The provided header is invalid or not in the expected format"),
            MyError::InvalidToken => write!(f, "The provided token is invalid"),
            MyError::MissingHeader => write!(f, "The required header is missing from the request"),
        }
    }
}

impl Error for MyError {}

impl ActixResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match self {
            // 3rd part error responses
            MyError::WebError(http_err) => HttpResponse::build(http_err.status).json(http_err.message.clone()),
            MyError::PoolError(_) | MyError::DBError(_) | MyError::AnyhowError(_) | MyError::BcryptError(_) | MyError::JsonError(_)
            | MyError::UuidError(_) | MyError::ActixError(_)
             => {HttpResponse::InternalServerError().json("Internal Server Error")},
            MyError::JwtError(_) => HttpResponse::Unauthorized().json("Invalid JWT"),

            // Query error responses
            MyError::UserNotFound => HttpResponse::Unauthorized().json("User not found"),

            // Request error responses
            MyError::MissingHeader => HttpResponse::BadRequest().json("Missing Authorization header"),
            MyError::InvalidHeader => HttpResponse::BadRequest().json("Invalid Authorization header format"),
            MyError::InvalidToken => HttpResponse::Unauthorized().json("Invalid Bearer token")
        }
    }
    
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::WebError(http_err) => http_err.status,
            MyError::JwtError(_) => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<ActixError> for MyError {
    fn from(err: ActixError) -> MyError {
        MyError::ActixError(err)
    }
}

impl From<AnyhowError> for MyError {
    fn from(err: AnyhowError) -> MyError {
        MyError::AnyhowError(err)
    }
}

impl From<BcryptError> for MyError {
    fn from(err: BcryptError) -> MyError {
        MyError::BcryptError(err)
    }
}

impl From<DbErr> for MyError {
    fn from(err: DbErr) -> MyError {
        MyError::DBError(err)
    }
}

impl From<JsonError> for MyError {
    fn from(err: JsonError) -> MyError {
        MyError::JsonError(err)
    }
}

impl From<JwtError> for MyError {
    fn from(err: JwtError) -> MyError {
        MyError::JwtError(err)
    }
}

impl From<SqlErr> for MyError {
    fn from(err: SqlErr) -> MyError {
        MyError::PoolError(err)
    }
}

impl From<UuidError> for MyError {
    fn from(err: UuidError) -> MyError {
        MyError::UuidError(err)
    }
}

impl From<HttpError> for MyError {
    fn from(err: HttpError) -> MyError {
        MyError::WebError(err)
    }
}
