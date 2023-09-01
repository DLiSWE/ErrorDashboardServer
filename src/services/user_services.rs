use diesel::r2d2::{ConnectionManager, Pool};
use diesel::prelude::*;
use diesel::PgConnection;
use uuid::Uuid;

use crate::models::User;
use crate::schema::users;
use crate::schema::users::dsl::*;

pub struct UserService {
    pub conn: Pool<ConnectionManager<PgConnection>>,
}

impl UserService {
    pub fn new(conn: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { conn }
    }

    pub fn create_user(&self, user_name: String, user_email: String, user_pass: String) -> Result<Uuid, Box<dyn std::error::Error>> {
        let uid = Uuid::new_v4();
        
        let user = User::new(uid, user_name, user_email, user_pass)
            .map_err(|_| "Error creating user model")?;
        
        let mut conn = self.conn.get().map_err(|_| "Couldn't get db connection from pool")?;
        
        diesel::insert_into(users::table)
            .values(&user)
            .execute(&mut conn)
            .map_err(|_| "Error saving new user")?;

        Ok(uid)
    }

    pub fn get_user(&self, uid: Uuid) -> Option<User> {
        let mut conn = self.conn.get().expect("Couldn't get db connection from pool");

        users::table.filter(id.eq(uid))
            .first::<User>(&mut conn)
            .ok()
    }

    pub fn delete_user(&self, uid: Uuid) -> bool {
        let mut conn = self.conn.get().expect("Couldn't get db connection from pool");

        diesel::delete(users::table.filter(id.eq(uid)))
            .execute(&mut conn)
            .is_ok()
    }
}
