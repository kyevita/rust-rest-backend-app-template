use crate::{config::Configuration, db::connection::Connection, routes};
use actix_web::web;

pub fn router(
    connection: Connection,
    configuration: Configuration,
) -> impl FnOnce(&mut web::ServiceConfig) {
    |cfg: &mut web::ServiceConfig| {
        cfg.service(web::scope("/api/v1").service(routes::users::route(connection, configuration)));
    }
}
