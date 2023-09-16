use sea_orm::{entity::prelude::*, ActiveModelTrait, EntityTrait, IntoActiveModel};
use chrono::Utc;
use uuid::Uuid;
use crate::models::user_model::{Entity as UserEntity, Model as UserModel};

pub struct UserService {
    pub db: DatabaseConnection,
}

impl UserService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, user_name: String, user_email: String, user_pass: String) -> Result<Uuid, sea_orm::error::DbErr> {
        let uid = Uuid::new_v4();
        
        let now = Utc::now().naive_local();

        let user = UserModel {
            id: uid,
            username: user_name,
            email: user_email,
            password: user_pass,
            created_at: now,
            updated_at: None
        }.into_active_model();

        UserEntity::insert(user).exec(&self.db).await?;
        
        Ok(uid)
    }

    pub async fn get_user(&self, uid: Uuid) -> Option<UserModel> {
        UserEntity::find_by_id(uid)
            .one(&self.db).await.ok()
            .flatten()
    }

    pub async fn delete_user(&self, uid: Uuid) -> Result<(), sea_orm::error::DbErr> {
        UserEntity::delete_by_id(uid)
            .exec(&self.db).await?;
        Ok(())
    }
}
