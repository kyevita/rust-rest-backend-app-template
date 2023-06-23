use actix_web::{web, HttpResponse};

use crate::{db::models::user::User, exceptions::ControllerError, services::users::UserService};

pub async fn add_user(user_service: web::Data<UserService>, form: web::Json<User>) -> HttpResponse {
    let service_instance = user_service.into_inner();
    let result = service_instance.create_user(form.into_inner()).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_user(
    user_service: web::Data<UserService>,
    username: web::Path<String>,
) -> HttpResponse {
    let result = user_service.get_user_by_username(&username).await;

    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(ControllerError {
            message: "User with username {username} not found",
        }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
