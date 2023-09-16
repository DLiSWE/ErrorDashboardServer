use actix_web::{web, HttpResponse, Result};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::dtos::user_dtos::UserCreateDTO;
use crate::services::UserService;

pub struct UserHandler;

impl UserHandler {

    pub async fn create_user(
        db: web::Data<DatabaseConnection>,
        new_user: web::Json<UserCreateDTO>,
    ) -> Result<HttpResponse, actix_web::Error> {
        let user_service = UserService::new(db.as_ref().clone());
        
        match user_service.create_user(new_user.username.clone(), new_user.email.clone(), new_user.password.clone()).await {
            Ok(id) => Ok(HttpResponse::Ok().json(id)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        }
    }

    pub async fn get_user(
        db: web::Data<DatabaseConnection>,
        user_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, actix_web::Error> {
        let user_service = UserService::new(db.as_ref().clone());

        match user_service.get_user(user_id.into_inner()).await {
            Some(user) => Ok(HttpResponse::Ok().json(user)),
            None => Ok(HttpResponse::NotFound().into()),
        }
    }

    pub async fn delete_user(
        db: web::Data<DatabaseConnection>,
        user_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, actix_web::Error> {
        let user_service = UserService::new(db.as_ref().clone());

        match user_service.delete_user(user_id.into_inner()).await {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(_) => Ok(HttpResponse::NotFound().finish()),
        }
    }
}

