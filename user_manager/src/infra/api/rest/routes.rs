use crate::infra::api::rest::handlers;
use actix_web::web;

pub fn api_routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/users")
                .route("", web::post().to(handlers::create_user))
                .route("", web::get().to(handlers::list_users))
                .route("/{id}", web::get().to(handlers::get_user_by_id))
                .route("/{id}", web::put().to(handlers::update_user))
                .route("/{id}", web::delete().to(handlers::delete_user)),
        ),
    );
}
