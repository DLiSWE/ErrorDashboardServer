use actix_web::http::{StatusCode, header::HeaderMap};
use jsonwebtoken::{Validation, TokenData, decode, DecodingKey};
use sea_orm::{EntityTrait, DatabaseConnection};
use uuid::Uuid;

use crate::dtos::user_dtos::ShortUserDTO;
use crate::middlewares::auth_middleware::Claims;
use crate::models::user_model::{Entity as UserEntity, Model as UserModel};
use crate::shared::utils::errors::{MyError, HttpError};

pub async fn validate_jwt(headers: &HeaderMap, secret_key: &str, validation: &Validation, db: &DatabaseConnection) -> Result<(), MyError> {
    if let Some(token_header) = headers.get("Authorization") {
        let token_str = token_header.to_str().unwrap_or("");

        let decoding_key = DecodingKey::from_secret(secret_key.as_ref());

        let token_data : TokenData<Claims> = decode(token_str, &decoding_key, &validation).map_err(MyError::from)?;

        let uid = token_data.claims.sub.parse::<Uuid>()
            .map_err(|_| MyError::WebError(HttpError {
                status: StatusCode::UNAUTHORIZED,
                message: "Invalid subject".to_string(),
            }))?;

        let found_user = UserEntity::find_by_id(uid)
            .one(db).await
            .map_err(|_| MyError::WebError(HttpError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Database error".to_string(),
            }))?;

        if let Some(_found_user) = found_user {
        // Authentication success
            return Ok(());
        } else {
        // Authentication failed
            return Err(MyError::UserNotFound);
        }   

    } else {
        return Err(MyError::WebError(HttpError {
            status: StatusCode::UNAUTHORIZED,
            message: "No Authorization header".to_string(),
        }));
    }
}

// pub fn create_access_token(user_id: String) -> Result<String, MyError> {
//     let claims = Claims {
//         sub:user_id,
//         iat:,
//         exp:,
//         iss:,
//         aud:,
//         data:,
//     }

//     return Ok("")
// }
