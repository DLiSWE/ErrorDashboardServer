use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Result};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::shared::utils::errors::{MyError, HttpError};
use crate::services::UserService;

pub struct UserHandler;

impl UserHandler {
    pub async fn get_user(
        db: web::Data<DatabaseConnection>,
        user_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, MyError> {
        let user_service = UserService::new(db.as_ref().clone());

        match user_service.get_user(user_id.into_inner()).await {
            Some(user) => Ok(HttpResponse::Ok().json(user)),
            None => Err(MyError::WebError(HttpError {
                status: StatusCode::NOT_FOUND,
                message: "User not found".to_string(),
            }))
        }
    }

    pub async fn delete_user(
        db: web::Data<DatabaseConnection>,
        user_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, MyError> {
        let user_service = UserService::new(db.as_ref().clone());

        match user_service.delete_user(user_id.into_inner()).await {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(_) => Err(MyError::WebError(HttpError {
                status: StatusCode::NOT_FOUND,
                message: "Could not delete user".to_string(),
            }))
        }
    }
}

