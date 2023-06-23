use crate::config::Configuration;
use crate::controllers::users;
use crate::db::connection::Connection;
use crate::services::users::UserService;
use actix_web::web::{self, get, post, Data};

pub fn route(connection: Connection, configuration: Configuration) -> actix_web::Scope {
    let user_service = UserService::new(connection, configuration);

    return web::scope("/users")
        .app_data(Data::new(user_service))
        .route("", post().to(users::add_user))
        .route("/{username}", get().to(users::get_user));
}
