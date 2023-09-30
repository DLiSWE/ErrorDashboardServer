use actix_web::cookie::{Cookie, SameSite};
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, HttpRequest, Result};
use sea_orm::DatabaseConnection;
use serde_json::to_string;
use std::sync::Arc;

use crate::config::Config;
use crate::dtos::user_dtos::{UserLoginDTO, UserCreateDTO};
use crate::services::AuthService;
use crate::shared::utils::errors::{MyError, HttpError};

pub struct AuthHandler;

impl AuthHandler {
    pub async fn login(db: web::Data<Arc<DatabaseConnection>>, config: web::Data<Arc<Config>>, login_data: web::Json<UserLoginDTO>) -> Result<HttpResponse, MyError> {
        let config = config.get_ref().clone();
        let auth_services = AuthService::new(db.as_ref().clone(), config)?;
        let UserLoginDTO { email, password } = login_data.into_inner();

        match auth_services.login(email, password).await {
        Ok(user_response) => {
            let json_refresh = to_string(&user_response.refresh_token).map_err(MyError::JsonError)?;

            let cookie = Cookie::build("refresh_token", json_refresh)
                .http_only(true)
                .secure(false)
                .same_site(SameSite::Strict)
                .finish();
            Ok(HttpResponse::Ok().cookie(cookie).json(user_response))
        },
        Err(err) => Err(err),
    }
    }

    pub async fn register(db: web::Data<Arc<DatabaseConnection>>, config: web::Data<Arc<Config>>, new_user: web::Json<UserCreateDTO>) -> Result<HttpResponse, MyError> {
        let config = config.get_ref().clone();
        let auth_services = AuthService::new(db.as_ref().clone(), config)?;
        
        match auth_services.register(new_user.username.clone(), new_user.email.clone(), new_user.password.clone()).await {
            Ok(id) => Ok(HttpResponse::Ok().json(id)),
            Err(MyError::WebError(http_err)) => Ok(http_err.into()),
            Err(_) => Err(MyError::WebError(HttpError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Internal server error".to_string(),
            })),
        }
    }

    pub async fn refresh_access_token(req: HttpRequest,db: web::Data<Arc<DatabaseConnection>>, config: web::Data<Arc<Config>>) -> Result<HttpResponse, MyError> {
        let auth_header = req.headers().get("Authorization");
        
        match auth_header {
            Some(header) => {
                let header_str = header.to_str().map_err(|_| MyError::InvalidHeader)?;
                let token = header_str
                    .strip_prefix("Bearer ")
                    .ok_or(MyError::InvalidToken)?;

                let config = config.get_ref().clone();
                let auth_services = AuthService::new(db.as_ref().clone(), config)?;

                let access_token = auth_services.refresh_access_token(token.to_string()).await?;

                Ok(HttpResponse::Ok().json(access_token))
            }
            None => Err(MyError::MissingHeader)
        }
    }

}
