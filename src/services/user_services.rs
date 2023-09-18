use sea_orm::{entity::prelude::*, EntityTrait};
use uuid::Uuid;

use crate::models::user_model::{Entity as UserEntity, Model as UserModel};
use crate::shared::utils::errors::MyError;

pub struct UserService {
    pub db: DatabaseConnection,
}

impl UserService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_user(&self, uid: Uuid) -> Option<UserModel> {
        UserEntity::find_by_id(uid)
            .one(&self.db).await.ok()
            .flatten()
    }

    pub async fn delete_user(&self, uid: Uuid) -> Result<(), MyError> {
        UserEntity::delete_by_id(uid)
            .exec(&self.db).await?;
        Ok(())
    }
}
