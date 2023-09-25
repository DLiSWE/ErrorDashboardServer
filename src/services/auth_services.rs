use actix_web::http::StatusCode;
use sea_orm::{entity::prelude::*, EntityTrait, IntoActiveModel, Set};
use chrono::Utc;
use bcrypt::{verify, hash};
use uuid::Uuid;

use crate::models::user_model::{Entity as UserEntity, Model as UserModel};
use crate::models::refresh_token_model::{Entity as RefreshTokenEntity, Model as RefreshTokenModel};
use crate::dtos::user_dtos::{UserLoginResponse, ShortUserDTO};
use crate::shared::utils::errors::{MyError, HttpError};
use crate::shared::utils::jwt::{create_access_token, create_refresh_token};
use crate::config::Config;

pub struct AuthService {
    pub db: DatabaseConnection,
}

impl AuthService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn login(&self, user_email: String, user_password: String) -> Result<UserLoginResponse, MyError> {
        let configs = Config::from_env().map_err(MyError::from)?;

        let hash_cost = configs.hash_cost;
        let issuer = configs.jwt_issuer;
        let audience = configs.jwt_audience;

        let found_user: Option<UserModel> = UserEntity::find()
            .filter(<UserEntity as sea_orm::EntityTrait>::Column::Email
            .eq(user_email))
            .one(&self.db)
            .await.map_err(MyError::from)?;

        match found_user {
            Some(user) => {
                let is_valid = verify(&user_password, &hash_cost).map_err(MyError::from)?;
                if is_valid {
                    // Login successful
                    let access_token = create_access_token(user.clone())?;
                    let refresh_token_dto = create_refresh_token(user.id.to_string())?;

                    let refresh_token_model = RefreshTokenModel {
                        user_id: user.id.clone(),
                        token: refresh_token_dto.refresh_token,
                        issued_at: refresh_token_dto.issued_at,
                        expires_at: refresh_token_dto.expires_at,
                        issuer,
                        audience,
                        revoked: false,
                        id: Uuid::new_v4(),
                    }.into_active_model();

                    RefreshTokenEntity::insert(refresh_token_model)
                        .exec(&self.db)
                        .await?;

                    let user_response = UserLoginResponse { 
                        user: ShortUserDTO {
                        id: user.id.clone(),
                        username: user.username.clone(),
                        email: user.email.clone(),
                        },
                        access_token,
                    };

                    Ok(user_response)
                } else {
                    // Invalid password
                    Err(MyError::WebError(HttpError { status: StatusCode::BAD_REQUEST, message: "Invalid password".to_string() }))

                }
            },
            None => Err(MyError::WebError(HttpError { status: StatusCode::NOT_FOUND, message: "User not found".to_string() }))
        }
    }

    pub async fn register(&self, user_name: String, user_email: String, user_pass: String) -> Result<Uuid, MyError> {
        let configs = Config::from_env()?;

        let hash_cost = configs.hash_cost.parse().unwrap();

        let uid = Uuid::new_v4();
        
        let now = Utc::now().naive_local();

        let hashed_pass = hash(user_pass, hash_cost).unwrap();

        let user = UserModel {
            id: uid,
            username: user_name,
            email: user_email,
            password: hashed_pass,
            created_at: now,
            updated_at: None
        }.into_active_model();

        UserEntity::insert(user).exec(&self.db).await?;
        
        Ok(uid)
    }

}
