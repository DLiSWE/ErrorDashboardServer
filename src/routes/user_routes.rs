use actix_web::web;
use crate::handlers::user_handlers::UserHandler;


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/{id}", web::get().to(UserHandler::get_user))
            .route("/{id}", web::delete().to(UserHandler::delete_user))
    );
}
