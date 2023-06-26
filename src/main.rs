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
use log::{error, info};
use std::process::exit;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    let app_config = config::load_config();
    let app_host = std::env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let app_port = std::env::var("APP_PORT").unwrap_or_else(|_| "3000".into());
    let uri = std::env::var("DB_CONN_STR").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let db_name = std::env::var("DB_NAME").unwrap();
    let connect_to_mongo = Connection::new(uri, db_name).await;

    info!("STARTING APP AT {}:{}", app_host, app_port);

    let connection = match connect_to_mongo {
        Ok(connection) => connection,
        Err(e) => {
            error!("Error connection to Mongo: {}", e);
            exit(1);
        }
    };

    println!("{:#?}", app_config);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(connection.clone()))
            .app_data(web::Data::new(app_config.clone()))
            .configure(router::router(connection.clone(), app_config.clone()))
    })
    .bind((app_host, app_port.parse::<u16>().unwrap()))?
    .run()
    .await
}
