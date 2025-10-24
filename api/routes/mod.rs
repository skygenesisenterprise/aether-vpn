// Routes configuration module

use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::controllers;
use crate::utils::jwt;

pub fn init(cfg: &mut web::ServiceConfig) {
    let auth_middleware = HttpAuthentication::bearer(|req, credentials| async move {
        let jwt_secret = req.app_data::<web::Data<String>>().unwrap().as_ref().clone();
        match jwt::verify_jwt(credentials.token(), &jwt_secret) {
            Ok(user_id) => {
                req.extensions_mut().insert(user_id);
                Ok(req)
            }
            Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid token")),
        }
    });

    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health_check))
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(controllers::register))
                    .route("/login", web::post().to(controllers::login))
            )
            .service(
                web::scope("/users")
                    .wrap(auth_middleware.clone())
                    .route("", web::get().to(controllers::get_users))
                    .route("/{id}", web::get().to(controllers::get_user))
            )
            .service(
                web::scope("/servers")
                    .wrap(auth_middleware.clone())
                    .route("", web::get().to(controllers::get_servers))
                    .route("", web::post().to(controllers::create_server))
                    .route("/{id}", web::get().to(controllers::get_server))
                    .route("/location/{location}", web::get().to(controllers::get_servers_by_location))
            )
            .service(
                web::scope("/vpn")
                    .wrap(auth_middleware)
                    .route("/keys", web::post().to(|pool, user_id: web::ReqData<i32>| controllers::generate_keys(pool, *user_id)))
                    .route("/connect", web::post().to(|pool, user_id: web::ReqData<i32>, req| controllers::connect(pool, *user_id, req)))
                    .route("/connections", web::get().to(|pool, user_id: web::ReqData<i32>| controllers::get_connections(pool, *user_id)))
            )
    );
}

async fn health_check() -> actix_web::Result<impl actix_web::Responder> {
    Ok(actix_web::HttpResponse::Ok().json(serde_json::json!({"status": "ok"})))
}