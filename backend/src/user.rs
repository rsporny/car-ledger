use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct User {
    id: Uuid,
    name: String,
    email: String,
}

async fn create_user(user: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().json(user.into_inner())
}

async fn get_user(user_id: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().json(User {
        id: user_id.into_inner(),
        name: "Sample User".to_string(),
        email: "user@example.com".to_string(),
    })
}

async fn update_user(_user_id: web::Path<Uuid>, user: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().json(user.into_inner())
}

async fn delete_user(user_id: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().json(format!("Deleted user with id: {}", user_id))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/", web::post().to(create_user))
            .route("/{id}", web::get().to(get_user))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user)),
    );
}
