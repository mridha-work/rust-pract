mod core;
mod infra;

use actix_web::middleware::{Compress, Logger, NormalizePath};
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

use crate::core::entity::errors::DefaultError;
use crate::core::ports::service::UserServicePort;
use crate::core::services::users::UserService;
use crate::infra::api::rest::handlers;
use crate::infra::repository::sqlite::user_repository::SqliteUserRepository;

const DB_CONNECTION: &str = "user_manager.db";

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("ok!\n")
}

async fn default_handler() -> impl Responder {
    HttpResponse::NotFound().json(DefaultError::new("Path does not exist"))
}

fn api_routes_config(cfg: &mut web::ServiceConfig) {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    }

    let db_conn = Connection::open(DB_CONNECTION).unwrap();

    let query = r#"
        CREATE TABLE IF NOT EXISTS "users" (
            "id" INTEGER PRIMARY KEY
            , "name" TEXT NOT NULL
            , "email" TEXT NOT NULL UNIQUE
            , "created_at" TEXT DEFAULT CURRENT_TIMESTAMP
            , "updated_at" TEXT DEFAULT CURRENT_TIMESTAMP
        );
    "#;
    db_conn.execute(query, ()).unwrap();

    let db_connection = Arc::new(Mutex::new(db_conn));

    let user_repo = Arc::new(SqliteUserRepository::new(db_connection.clone()));
    let user_service: Arc<dyn UserServicePort> = Arc::new(UserService::new(user_repo));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(NormalizePath::default())
            .app_data(web::Data::new(user_service.clone()))
            .service(ping)
            .configure(api_routes_config)
            .default_service(web::route().to(default_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
