use clap::{Parser, Subcommand};
use anyhow::Result;

mod commands;
mod config;
mod error;

use commands::*;

#[derive(Parser)]
#[command(name = "avpn")]
#[command(about = "Aether VPN CLI client")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Authentication commands
    Auth {
        #[command(subcommand)]
        auth_command: AuthCommands,
    },
    /// Server management
    Servers {
        #[command(subcommand)]
        server_command: ServerCommands,
    },
    /// VPN operations
    Vpn {
        #[command(subcommand)]
        vpn_command: VpnCommands,
    },
    /// Configuration management
    Config {
        #[command(subcommand)]
        config_command: ConfigCommands,
    },
    /// Show user information
    User {
        #[command(subcommand)]
        user_command: UserCommands,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Auth { auth_command } => handle_auth(auth_command).await,
        Commands::Servers { server_command } => handle_servers(server_command).await,
        Commands::Vpn { vpn_command } => handle_vpn(vpn_command).await,
        Commands::Config { config_command } => handle_config(config_command).await,
        Commands::User { user_command } => handle_user(user_command).await,
    }
}