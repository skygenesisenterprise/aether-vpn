use clap::Subcommand;
use anyhow::Result;
use crate::config::Config;
use crate::api::ApiClient;

#[derive(Subcommand)]
pub enum ServerCommands {
    /// List all available servers
    List,
    /// Show server details
    Show {
        /// Server ID
        id: i32,
    },
}

pub async fn handle_servers(command: ServerCommands) -> Result<()> {
    let config = Config::load()?;

    if config.jwt_token.is_none() {
        eprintln!("You must be logged in to use this command. Run 'avpn auth login <email>' first.");
        return Ok(());
    }

    let client = ApiClient::new(config);

    match command {
        ServerCommands::List => {
            let servers = client.get_servers().await?;

            if servers.is_empty() {
                println!("No servers available");
            } else {
                println!("Available servers:");
                println!("{:<5} {:<20} {:<15} {}", "ID", "Name", "Location", "IP");
                println!("{}", "-".repeat(60));

                for server in servers {
                    println!("{:<5} {:<20} {:<15} {}",
                        server.id, server.name, server.location, server.ip);
                }
            }
        }
        ServerCommands::Show { id } => {
            let server = client.get_server(id).await?;

            println!("Server Details:");
            println!("ID: {}", server.id);
            println!("Name: {}", server.name);
            println!("Location: {}", server.location);
            println!("IP: {}", server.ip);
            println!("Public Key: {}", server.public_key);
            println!("Created: {}", server.created_at);
        }
    }

    Ok(())
}