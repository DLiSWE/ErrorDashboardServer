use bcrypt::BcryptError;
use chrono::Local;
use uuid::Uuid;
use crate::models::User;

pub struct UserService {
    users: Vec<User>,
}

impl UserService {
    pub fn new() -> Self {
        UserService { users: Vec::new() }
    }

    pub fn create_user(&mut self, username: String, email: String, plain_password: String) -> Result<Uuid, BcryptError> {
        let id = Uuid::new_v4();
        let _created_at = Local::now().naive_local();
        let user = User::new(id, username, email, plain_password)?;
        
        self.users.push(user);
        Ok(id)
    }

    pub fn get_user(&self, id: Uuid) -> Option<&User> {
        self.users.iter().find(|user| user.id == id)
    }

    pub fn delete_user(&mut self, id: Uuid) -> bool {
        if let Some(index) = self.users.iter().position(|user| user.id == id) {
            self.users.remove(index);
            true
        } else {
            false
        }
    }
}
