use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    // Skip serializing password
    #[serde(skip_serializing, skip_deserializing)]
    pub password: String,
}

impl User {
    pub fn new(id: i32, username: String, email: String, plain_password: String) -> Result<Self, bcrypt::BcryptError> {
        let hashed_password = hash(&plain_password, DEFAULT_COST)?;
        Ok(User {
            id,
            username,
            email,
            password: hashed_password,
        })
    }

    pub fn verify_password(&self, plain_password: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(plain_password, &self.password)
    }
}
