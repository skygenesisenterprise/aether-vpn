use actix_web::{web, App, HttpServer, HttpResponse, Result};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::io;

mod config;
mod controllers;
mod core;
mod middlewares;
mod models;
mod queries;
mod routes;
mod scripts;
mod services;
mod tests;
mod utils;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let config = config::Config::from_env();

    let manager = ConnectionManager::<PgConnection>::new(&config.database_url);
    let pool = Pool::builder().build(manager).expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.jwt_secret.clone()))
            .configure(routes::init)
    })
    .bind(format!("127.0.0.1:{}", config.server_port))?
    .run()
    .await
}
