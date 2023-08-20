use actix_web::{web, App};
use crate::routes::{user_routes};

pub fn create_app() -> App {
    App::new()
        .configure(user_routes::configure)
}
