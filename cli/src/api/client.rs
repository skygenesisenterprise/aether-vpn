use reqwest::Client;
use anyhow::Result;
use crate::config::Config;
use crate::api::models::*;
use crate::error::AvpnError;

pub struct ApiClient {
    client: Client,
    config: Config,
}

impl ApiClient {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<AuthResponse> {
        let request = LoginRequest {
            email: email.to_string(),
            password: password.to_string(),
        };

        let response = self.client
            .post(&format!("{}/api/auth/login", self.config.api_url))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(AvpnError::Auth("Invalid credentials".to_string()).into())
        }
    }

    pub async fn get_servers(&self) -> Result<Vec<Server>> {
        let response = self.client
            .get(&format!("{}/api/servers", self.config.api_url))
            .bearer_auth(self.config.jwt_token.as_ref().unwrap())
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(AvpnError::Api("Failed to get servers".to_string()).into())
        }
    }

    pub async fn get_server(&self, id: i32) -> Result<Server> {
        let response = self.client
            .get(&format!("{}/api/servers/{}", self.config.api_url, id))
            .bearer_auth(self.config.jwt_token.as_ref().unwrap())
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(AvpnError::Api("Server not found".to_string()).into())
        }
    }

    pub async fn connect(&self, server_id: i32) -> Result<ConnectResponse> {
        let request = ConnectRequest { server_id };

        let response = self.client
            .post(&format!("{}/api/vpn/connect", self.config.api_url))
            .bearer_auth(self.config.jwt_token.as_ref().unwrap())
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(AvpnError::Api("Failed to connect".to_string()).into())
        }
    }

    pub async fn get_user(&self) -> Result<User> {
        // This would need to be implemented based on your API
        // For now, we'll get user info from the JWT token
        unimplemented!("User info endpoint not yet implemented")
    }
}