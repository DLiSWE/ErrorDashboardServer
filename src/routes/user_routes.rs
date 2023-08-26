use actix_web::web;
use crate::handlers::user_handlers;


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(user_handlers::create_user))
            .route("/{id}", web::get().to(user_handlers::get_user))
            // .route("/{id}", web::delete().to(user_handlers::delete_user))
    );
}
