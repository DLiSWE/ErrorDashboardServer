use actix_web::{web, HttpResponse, Result};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use uuid::Uuid;

use crate::dtos::user_dtos::UserCreateDTO;
use crate::services::UserService;

pub struct UserHandler;

impl UserHandler {

    pub async fn create_user(
        pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
        new_user: web::Json<UserCreateDTO>,
    ) -> Result<HttpResponse, actix_web::Error> {
        let user_service = UserService::new(pool.as_ref().clone());
        
        match user_service.create_user(new_user.username.clone(), new_user.email.clone(), new_user.password.clone()) {
            Ok(id) => Ok(HttpResponse::Ok().json(id)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        }
    }

    pub async fn get_user(
        pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
        user_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, actix_web::Error> {
        let user_service = UserService::new(pool.as_ref().clone());

        match user_service.get_user(user_id.into_inner()) {
            Some(user) => Ok(HttpResponse::Ok().json(user)),
            None => Ok(HttpResponse::NotFound().into()),
        }
    }

    pub async fn delete_user(
        pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
        user_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, actix_web::Error> {
        let user_service = UserService::new(pool.as_ref().clone());

        if user_service.delete_user(user_id.into_inner()) {
            Ok(HttpResponse::Ok().finish())
        } else {
            Ok(HttpResponse::NotFound().finish())
        }
    }
}
