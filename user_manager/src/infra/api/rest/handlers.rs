use crate::core::entity::user::{CreateUserRequest, ListUsersQueryParam, UpdateUserRequest};
use crate::core::ports::service::UserServicePort;
use crate::infra::api::rest::errors::DefaultError;
use actix_web::{HttpResponse, Responder, get, web};
use std::sync::Arc;

pub async fn default_handler() -> impl Responder {
    HttpResponse::NotFound().json(DefaultError::new("Path does not exist"))
}

#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("ok!\n")
}

pub async fn create_user(
    service: web::Data<Arc<dyn UserServicePort>>,
    req: web::Json<CreateUserRequest>,
) -> impl Responder {
    match service.create_user(req.into_inner()) {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => HttpResponse::BadRequest().json(DefaultError::new(e)),
    }
}

pub async fn get_user_by_id(
    service: web::Data<Arc<dyn UserServicePort>>,
    id: web::Path<i64>,
) -> impl Responder {
    match service.get_user_by_id(*id) {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(DefaultError::new("User not found")),
        Err(e) => HttpResponse::InternalServerError().json(DefaultError::new(e)),
    }
}

pub async fn list_users(
    service: web::Data<Arc<dyn UserServicePort>>,
    query: web::Query<ListUsersQueryParam>,
) -> impl Responder {
    match service.list_users(query.into_inner()) {
        Ok(user_list) => HttpResponse::Ok().json(user_list),
        Err(e) => HttpResponse::InternalServerError().json(DefaultError::new(e)),
    }
}

pub async fn update_user(
    service: web::Data<Arc<dyn UserServicePort>>,
    id: web::Path<i64>,
    req: web::Json<UpdateUserRequest>,
) -> impl Responder {
    match service.update_user(*id, req.into_inner()) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::BadRequest().json(DefaultError::new(e)),
    }
}

pub async fn delete_user(
    service: web::Data<Arc<dyn UserServicePort>>,
    id: web::Path<i64>,
) -> impl Responder {
    match service.delete_user(*id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(DefaultError::new(e)),
    }
}
