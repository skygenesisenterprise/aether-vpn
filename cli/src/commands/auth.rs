use clap::Subcommand;
use anyhow::Result;
use crate::config::Config;
use crate::api::ApiClient;
use std::io::{self, Write};

#[derive(Subcommand)]
pub enum AuthCommands {
    /// Login to Aether VPN
    Login {
        /// Email address
        email: String,
    },
    /// Logout from Aether VPN
    Logout,
}

pub async fn handle_auth(command: AuthCommands) -> Result<()> {
    match command {
        AuthCommands::Login { email } => {
            print!("Password: ");
            io::stdout().flush()?;

            let password = rpassword::read_password()?;

            let mut config = Config::load()?;
            let client = ApiClient::new(config.clone());

            let auth_response = client.login(&email, &password).await?;

            config.jwt_token = Some(auth_response.token);
            config.save()?;

            println!("Successfully logged in as {}", auth_response.user.username);
        }
        AuthCommands::Logout => {
            let mut config = Config::load()?;
            config.jwt_token = None;
            config.save()?;
            println!("Successfully logged out");
        }
    }

    Ok(())
}