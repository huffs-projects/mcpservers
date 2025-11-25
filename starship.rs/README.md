# Starship Rust MCP Server

A Rust-native MCP server providing structured, agentic access to Starship prompt configuration.

## Features

- **Query Options**: Query all Starship configuration options with type, default, category, and documentation
- **Presets**: Access available Starship presets with structured snippets
- **Templates**: Generate configuration snippets based on category or use case
- **Validation**: Validate TOML configuration files against schema
- **Safe Application**: Apply configuration changes with dry-run, backup, and logging

## Installation

```bash
cargo build --release
```

## Usage

Start the server:

```bash
PORT=8080 cargo run
```

Or use the default port (8080):

```bash
cargo run
```

## API Endpoints

### Health Check

```bash
curl http://localhost:8080/health
```

### MCP Endpoints

All MCP endpoints are available at `/mcp` via POST requests.

#### starship_options

Query Starship configuration options.

```bash
curl -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "method": "starship_options",
    "params": {
      "search_term": "git",
      "category": "module"
    }
  }'
```

#### starship_presets

Get available Starship presets.

```bash
curl -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "method": "starship_presets",
    "params": {
      "preset_name": "no-runtime-versions"
    }
  }'
```

#### starship_templates

Generate configuration templates.

```bash
curl -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "method": "starship_templates",
    "params": {
      "category": "git",
      "use_case": "development"
    }
  }'
```

#### starship_validate

Validate a Starship configuration file.

```bash
curl -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "method": "starship_validate",
    "params": {
      "config_path": "/path/to/starship.toml"
    }
  }'
```

#### starship_apply

Apply configuration changes safely.

```bash
curl -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "method": "starship_apply",
    "params": {
      "config_path": "/path/to/starship.toml",
      "patch": "[git_branch]\nformat = \"$branch\"",
      "dry_run": true,
      "backup_path": "/path/to/backup.toml"
    }
  }'
```

## Documentation

This server is fully grounded in authoritative Starship sources:

- [Configuration](https://starship.rs/config/)
- [Advanced Config](https://starship.rs/advanced-config/)
- [Presets](https://starship.rs/presets/)

## Project Structure

```
src/
├── endpoints/          # MCP endpoint implementations
│   ├── starship_options.rs
│   ├── starship_presets.rs
│   ├── starship_templates.rs
│   ├── starship_validate.rs
│   └── starship_apply.rs
├── models/             # Data models
│   └── mod.rs
├── utils/              # Utility modules
│   ├── file.rs         # File operations with locking
│   ├── logger.rs        # Structured logging
│   └── parser.rs        # TOML parsing
├── server.rs           # HTTP server (warp)
└── main.rs             # Entry point
```

## Guidelines

- Always perform dry-run before applying changes
- Backups are created automatically before mutations
- All options, templates, and presets reference official Starship documentation URLs
- Structured JSON outputs suitable for agent reasoning

## License

MIT

