use actix_web::http::StatusCode;
use sea_orm::{entity::prelude::*, ActiveModelTrait, EntityTrait, IntoActiveModel};
use chrono::Utc;
use bcrypt::{verify, hash};

use crate::models::user_model::{Entity as UserEntity, Model as UserModel};
use crate::dtos::user_dtos::UserResponseDTO;
use crate::shared::utils::errors::{MyError, HttpError};
use crate::config::Config;

pub struct AuthService {
    pub db: DatabaseConnection,
}

impl AuthService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn login(&self, user_email: String, user_password: String) -> Result<UserResponseDTO, MyError> {
        let configs = Config::from_env().map_err(MyError::from)?;

        let hash_cost = configs.hash_cost;

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
                    let user_response = UserResponseDTO {
                        id: user.id.clone(),
                        username: user.username.clone(),
                        email: user.email.clone(),
                    };
                    Ok(user_response)
                } else {
                    // Invalid password
                    Err(MyError::WebError(HttpError { status: StatusCode::BAD_REQUEST, message: "Invalid password".to_string() }))

                }
            },
            None => Err(MyError::WebError(HttpError { status: StatusCode::BAD_REQUEST, message: "User not".to_string() }))
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
