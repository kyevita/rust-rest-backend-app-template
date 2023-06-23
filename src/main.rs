// Modules
mod config;
mod controllers;
mod db;
mod exceptions;
mod router;
mod routes;
mod services;

// Lib
use actix_web::{web, App, HttpServer};
use db::connection::Connection;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();

    let app_config = config::load_config();
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let db_name = std::env::var("DB_NAME").unwrap();
    let connection: Connection = Connection::new(uri, db_name).await;

    println!("{:#?}", app_config);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(connection.clone()))
            .app_data(web::Data::new(app_config.clone()))
            .configure(router::router(connection.clone(), app_config.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
