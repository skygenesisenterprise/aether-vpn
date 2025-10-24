use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::models::User;
use crate::queries;
use crate::utils::jwt;
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    token: String,
    user: User,
}

pub async fn register(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    jwt_secret: web::Data<String>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let password_hash = hash(req.password.clone(), DEFAULT_COST).unwrap();
    let user = queries::create_user(&conn, &req.username, &req.email, &password_hash)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create user"))?;
    let token = jwt::create_jwt(user.id, &jwt_secret)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create token"))?;
    Ok(HttpResponse::Ok().json(AuthResponse { token, user }))
}

pub async fn login(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    jwt_secret: web::Data<String>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let user = queries::get_user_by_email(&conn, &req.email)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid credentials"))?;
    if verify(&req.password, &user.password_hash).unwrap() {
        let token = jwt::create_jwt(user.id, &jwt_secret)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create token"))?;
        Ok(HttpResponse::Ok().json(AuthResponse { token, user }))
    } else {
        Err(actix_web::error::ErrorUnauthorized("Invalid credentials"))
    }
}

pub async fn get_users(pool: web::Data<Pool<ConnectionManager<PgConnection>>>) -> Result<HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let users = queries::get_all_users(&conn)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get users"))?;
    Ok(HttpResponse::Ok().json(users))
}

pub async fn get_user(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<i32>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");
    let user = queries::get_user(&conn, user_id)
        .map_err(|_| actix_web::error::ErrorNotFound("User not found"))?;
    Ok(HttpResponse::Ok().json(user))
}