use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Result};
use sea_orm::DatabaseConnection;

use crate::dtos::user_dtos::{UserLoginDTO, UserCreateDTO};
use crate::services::AuthService;
use crate::shared::utils::errors::{MyError, HttpError};

pub struct AuthHandler;

impl AuthHandler {

    pub async fn login(
        db: web::Data<DatabaseConnection>,
        login_data: web::Json<UserLoginDTO>,
    )-> Result<HttpResponse, MyError> {
        
        let auth_services = AuthService::new(db.as_ref().clone());

        let UserLoginDTO { email, password } = login_data.into_inner();

        match auth_services.login(email, password).await {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(e) => Err(e),
       }
    }
    pub async fn register(
        db: web::Data<DatabaseConnection>,
        new_user: web::Json<UserCreateDTO>,
    ) -> Result<HttpResponse, MyError> {
        let auth_services = AuthService::new(db.as_ref().clone());

        match auth_services.register(new_user.username.clone(), new_user.email.clone(), new_user.password.clone()).await {
            Ok(id) => Ok(HttpResponse::Ok().json(id)),
            Err(MyError::WebError(http_err)) => Ok(http_err.into()),
            Err(_) => Err(MyError::WebError(HttpError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Internal server error".to_string(),
            })),
        }
    }
}
