# Aether VPN CLI

A command-line client for Aether VPN.

## Installation

```bash
cargo build --release
# Copy target/release/avpn to your PATH
```

## Usage

### Authentication
```bash
# Login
avpn auth login user@example.com

# Logout
avpn auth logout
```

### Server Management
```bash
# List servers
avpn servers list

# Show server details
avpn servers show 1
```

### VPN Operations
```bash
# Connect to server
avpn vpn connect 1

# Check status
avpn vpn status

# Disconnect
avpn vpn disconnect
```

### Configuration
```bash
# Set API URL
avpn config set-api-url https://api.aether-vpn.com

# Show config
avpn config show
```

## Requirements

- WireGuard tools (`wg`, `wg-quick`)
- Linux/macOS/Windows with WireGuard support