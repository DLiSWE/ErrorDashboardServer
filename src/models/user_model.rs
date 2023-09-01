use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::schema::users;

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl User {
    pub fn new(id: Uuid, username: String, email: String, password: String) -> Result<Self, bcrypt::BcryptError> {
        let current_time = chrono::Local::now().naive_local(); 

        let hashed_password = hash(&password, DEFAULT_COST)?;
        Ok(User {
            id,
            username,
            email,
            password: hashed_password,
            created_at: current_time,
            updated_at: None,
        })
    }

    pub fn verify_password(&self, plain_password: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(plain_password, &self.password)
    }
}
