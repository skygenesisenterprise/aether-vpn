use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::models::Server;
use crate::queries;
use crate::services::vpn_service;
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateServerRequest {
    name: String,
    location: String,
    ip: String,
}

pub async fn create_server(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    req: web::Json<CreateServerRequest>,
) -> Result<HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let (_priv, pub_key) = vpn_service::generate_keypair();
    let server = queries::create_server(&conn, &req.name, &req.location, &req.ip, &pub_key)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create server"))?;
    Ok(HttpResponse::Created().json(server))
}

pub async fn get_servers(pool: web::Data<Pool<ConnectionManager<PgConnection>>>) -> Result<HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let servers = queries::get_all_servers(&conn)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get servers"))?;
    Ok(HttpResponse::Ok().json(servers))
}

pub async fn get_server(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<i32>,
) -> Result<HttpResponse> {
    let server_id = path.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");
    let server = queries::get_server(&conn, server_id)
        .map_err(|_| actix_web::error::ErrorNotFound("Server not found"))?;
    Ok(HttpResponse::Ok().json(server))
}

pub async fn get_servers_by_location(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    location: web::Path<String>,
) -> Result<HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let servers = queries::get_servers_by_location(&conn, &location)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get servers"))?;
    Ok(HttpResponse::Ok().json(servers))
}