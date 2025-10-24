use clap::Subcommand;
use anyhow::Result;

#[derive(Subcommand)]
pub enum UserCommands {
    /// Show user information
    Info,
}

pub async fn handle_user(command: UserCommands) -> Result<()> {
    match command {
        UserCommands::Info => {
            println!("User info command not yet implemented");
        }
    }

    Ok(())
}