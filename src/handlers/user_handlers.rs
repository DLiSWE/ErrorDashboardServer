use actix_web::{web, HttpResponse, Responder, Result};
use std::sync::{Arc,Mutex};
use crate::models::User;
use crate::services::UserService;
use uuid::Uuid;

pub async fn create_user(user_service: web::Data<Arc<Mutex<UserService>>>, info: web::Json<User>) -> Result<impl Responder> {
    let mut user_service = user_service.lock().unwrap();

    let new_user = user_service.create_user(info.username.clone(),
        info.email.clone(),
        info.password.clone())
        .map_err(|_err| actix_web::error::ErrorInternalServerError("Internal Server Error"))?;

    Ok(HttpResponse::Created().json(new_user))
}

pub async fn get_user(user_service: web::Data<UserService>, user_id: web::Path<Uuid>) -> Result<impl Responder> {
    if let Some(user) = user_service.get_user(*user_id) {
        Ok(HttpResponse::Ok().json(user))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

// pub async fn delete_user(user_service: web::Data<Arc<Mutex<UserService>>>, user_id: web::Path<Uuid>) -> Result<impl Responder> {
//     if user_service.delete_user(*user_id) {
//         Ok(HttpResponse::NoContent().finish())
//     } else {
//         Ok(HttpResponse::NotFound().finish())
//     }

