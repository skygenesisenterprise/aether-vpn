use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub ip: String,
    pub public_key: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnKey {
    pub id: i32,
    pub user_id: i32,
    pub server_id: Option<i32>,
    pub private_key: String,
    pub public_key: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub id: i32,
    pub user_id: i32,
    pub server_id: i32,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize)]
pub struct ConnectRequest {
    pub server_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct ConnectResponse {
    pub config: String,
    pub connection: Connection,
}