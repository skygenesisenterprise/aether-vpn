use clap::Subcommand;
use anyhow::Result;
use crate::config::Config;
use crate::api::ApiClient;
use crate::wireguard;

#[derive(Subcommand)]
pub enum VpnCommands {
    /// Connect to a VPN server
    Connect {
        /// Server ID to connect to
        server_id: i32,
    },
    /// Disconnect from VPN
    Disconnect,
    /// Show VPN connection status
    Status,
    /// Generate new VPN keys
    Keys,
}

pub async fn handle_vpn(command: VpnCommands) -> Result<()> {
    let config = Config::load()?;

    if config.jwt_token.is_none() {
        eprintln!("You must be logged in to use this command. Run 'avpn auth login <email>' first.");
        return Ok(());
    }

    let client = ApiClient::new(config);

    match command {
        VpnCommands::Connect { server_id } => {
            println!("Connecting to server {}...", server_id);

            let connect_response = client.connect(server_id).await?;

            // Save WireGuard config and apply it
            wireguard::apply_config(&connect_response.config)?;

            println!("Successfully connected to VPN!");
            println!("Connection ID: {}", connect_response.connection.id);
        }
        VpnCommands::Disconnect => {
            wireguard::disconnect()?;
            println!("Successfully disconnected from VPN");
        }
        VpnCommands::Status => {
            if let Some(status) = wireguard::get_status()? {
                println!("VPN Status: Connected");
                println!("{}", status);
            } else {
                println!("VPN Status: Disconnected");
            }
        }
        VpnCommands::Keys => {
            // This would call the API to generate keys
            println!("Key generation not yet implemented");
        }
    }

    Ok(())
}