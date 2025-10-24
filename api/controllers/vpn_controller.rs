use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::models::{VpnKey, Connection};
use crate::queries;
use crate::services::vpn_service;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ConnectRequest {
    server_id: i32,
}

#[derive(Serialize)]
struct ConnectResponse {
    config: String,
    connection: Connection,
}

pub async fn generate_keys(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    user_id: i32,
) -> Result<HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let (priv_key, pub_key) = vpn_service::generate_keypair();
    let vpn_key = queries::create_vpn_key(&conn, user_id, None, &priv_key, &pub_key)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create keys"))?;
    Ok(HttpResponse::Ok().json(vpn_key))
}

pub async fn connect(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    user_id: i32,
    req: web::Json<ConnectRequest>,
) -> Result<HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let server = queries::get_server(&conn, req.server_id)
        .map_err(|_| actix_web::error::ErrorNotFound("Server not found"))?;
    let keys = queries::get_vpn_keys_for_user(&conn, user_id)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get keys"))?;
    if keys.is_empty() {
        return Err(actix_web::error::ErrorBadRequest("No VPN keys found for user"));
    }
    let key = &keys[0]; // Assume first key
    let client_ip = format!("10.0.0.{}", user_id % 255 + 1);
    let config = vpn_service::generate_wireguard_config(&key.private_key, &server.ip, &server.public_key, &client_ip);
    let connection = queries::create_connection(&conn, user_id, req.server_id, "connected")
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create connection"))?;
    Ok(HttpResponse::Ok().json(ConnectResponse { config, connection }))
}

pub async fn get_connections(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    user_id: i32,
) -> Result<HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    // For simplicity, get connections for user
    // Actually, connections table has user_id, so filter
    use crate::schema::connections::dsl::*;
    let user_connections = connections.filter(user_id.eq(user_id)).load::<Connection>(&conn)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get connections"))?;
    Ok(HttpResponse::Ok().json(user_connections))
}