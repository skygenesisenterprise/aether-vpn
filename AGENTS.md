# Aether VPN - Agent Guidelines

## Build Commands

### Frontend (Next.js)
- `pnpm dev` - Start development server
- `pnpm build` - Build for production  
- `pnpm lint` - Run ESLint
- `pnpm start` - Start production server

### Rust Components
- `cargo build` - Build all workspace members
- `cargo test` - Run all tests
- `cargo test --package aether-vpn-api` - Run API tests
- `cargo test --package avpn` - Run CLI tests
- `cargo run --bin aether-vpn-api` - Run API server
- `cargo run --bin avpn` - Run CLI

## Code Style Guidelines

### TypeScript/React
- Use strict TypeScript configuration
- Import React components with `import Component from "./component"`
- Use Tailwind CSS classes for styling
- Follow Next.js App Router conventions
- Use `@/*` path aliases for internal imports

### Rust
- Use `rustfmt` for formatting
- Prefer `anyhow::Result<T>` for error handling in CLI
- Use `thiserror` for custom error types
- Follow diesel ORM patterns for database operations
- Use async/await with tokio runtime
- Structure: modules in separate files, main logic in `main.rs`

### General
- Use conventional commit messages
- Keep functions small and focused
- Prefer explicit error handling over panics
- Use environment variables for configuration