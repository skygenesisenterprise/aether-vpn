use clap::Subcommand;
use anyhow::Result;
use crate::config::Config;

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Set API URL
    SetApiUrl {
        /// API URL
        url: String,
    },
    /// Show current configuration
    Show,
}

pub async fn handle_config(command: ConfigCommands) -> Result<()> {
    let mut config = Config::load()?;

    match command {
        ConfigCommands::SetApiUrl { url } => {
            config.api_url = url;
            config.save()?;
            println!("API URL updated successfully");
        }
        ConfigCommands::Show => {
            println!("Current Configuration:");
            println!("API URL: {}", config.api_url);
            println!("Logged in: {}", config.jwt_token.is_some());
        }
    }

    Ok(())
}