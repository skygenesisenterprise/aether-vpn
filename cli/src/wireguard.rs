use std::process::Command;
use anyhow::Result;
use crate::error::AvpnError;

pub fn apply_config(config: &str) -> Result<()> {
    // Save config to temporary file
    let config_path = "/tmp/avpn.conf";
    std::fs::write(config_path, config)?;

    // Apply config using wg-quick
    let output = Command::new("wg-quick")
        .args(&["up", config_path])
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(AvpnError::WireGuard(format!("Failed to apply config: {}", error)).into())
    }
}

pub fn disconnect() -> Result<()> {
    let output = Command::new("wg-quick")
        .args(&["down", "/tmp/avpn.conf"])
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(AvpnError::WireGuard(format!("Failed to disconnect: {}", error)).into())
    }
}

pub fn get_status() -> Result<Option<String>> {
    let output = Command::new("wg")
        .arg("show")
        .output()?;

    if output.status.success() {
        let status = String::from_utf8_lossy(&output.stdout);
        if status.trim().is_empty() {
            Ok(None)
        } else {
            Ok(Some(status.to_string()))
        }
    } else {
        Ok(None)
    }
}