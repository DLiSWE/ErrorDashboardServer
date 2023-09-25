use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreateDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShortUserDTO {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponse {
    pub user: ShortUserDTO,
    pub access_token: String,
}
