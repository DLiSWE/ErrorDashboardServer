use actix_web::{web, HttpResponse};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to(|| async { HttpResponse::Ok().body("All users!") }))
            .route(web::post().to(|| async { HttpResponse::Ok().body("Create a user!") }))
    );
}
