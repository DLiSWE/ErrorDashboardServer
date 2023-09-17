use actix_web::{web, HttpResponse, Result, Error};
use actix_web::error::ErrorInternalServerError;
use sea_orm::DatabaseConnection;

use crate::dtos::user_dtos::{UserLoginDTO, UserCreateDTO};
use crate::services::AuthService;
use crate::shared::utils::errors::MyError;

pub struct AuthHandler;

impl AuthHandler {

    pub async fn login(
        db: web::Data<DatabaseConnection>,
        login_data: web::Json<UserLoginDTO>,
    )-> Result<HttpResponse, Error> {
        
        let auth_services = AuthService::new(db.as_ref().clone());

        let UserLoginDTO { email, password } = login_data.into_inner();

        match auth_services.login(email, password).await {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(MyError::WebError(http_err)) => Ok(http_err.into()),
            Err(_) => Err(ErrorInternalServerError("Internal server error")),
        }
    }

    pub async fn register(
        db: web::Data<DatabaseConnection>,
        new_user: web::Json<UserCreateDTO>,
    ) -> Result<HttpResponse, Error> {
        let auth_services = AuthService::new(db.as_ref().clone());

        match auth_services.register(new_user.username.clone(), new_user.email.clone(), new_user.password.clone()).await {
            Ok(id) => Ok(HttpResponse::Ok().json(id)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        }
    }

}