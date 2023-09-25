use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRefreshTokenDTO {
    pub refresh_token: String,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub jwt_iss: String,
    pub jwt_aud: String,
    pub revoked: bool
}