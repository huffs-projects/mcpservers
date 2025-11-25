# Mako Rust MCP Server

A Rust-native MCP (Model Context Protocol) server to programmatically manage Mako configuration (`mako/config`).

This server enables agents to query configuration options, generate template snippets, validate settings, and apply changes safely. Based on the official Mako source repository: https://github.com/emersion/mako.

## Features

- **Query Configuration Options**: List all Mako configuration options with types, defaults, and valid values
- **Generate Templates**: Get pre-configured templates for common use cases (minimal, persistent, colored, positional, etc.)
- **Validate Configuration**: Check Mako config files for syntax and semantic correctness
- **Apply Patches Safely**: Apply configuration changes with dry-run support, automatic backups, and diff generation

## Installation

### Prerequisites

- Rust 1.70+ (install from https://rustup.rs/)
- Cargo (comes with Rust)

### Build

```bash
cargo build --release
```

### Run

```bash
# Default port 8080
cargo run

# Custom port
PORT=3000 cargo run
```

## API Endpoints

### Query Endpoints

#### `GET /mako_options`

List Mako configuration options.

**Query Parameters:**
- `search_term` (optional): Filter options by name or description

**Example:**
```bash
curl "http://localhost:8080/mako_options?search_term=color"
```

**Response:**
```json
{
  "result": [
    {
      "name": "background-color",
      "type": "color",
      "default": "#285577",
      "description": "Background color for notifications",
      "valid_values": null,
      "documentation_url": "https://github.com/emersion/mako#background-color"
    }
  ]
}
```

#### `GET /mako_templates`

Generate Mako config snippets for common use cases.

**Query Parameters:**
- `use_case` (optional): Filter by template name (e.g., "minimal", "persistent", "colored", "positional")

**Example:**
```bash
curl "http://localhost:8080/mako_templates?use_case=minimal"
```

**Response:**
```json
{
  "result": [
    {
      "template_name": "minimal",
      "snippet": "[default]\nfont=monospace 10\nbackground-color=#285577\ntext-color=#ffffff\n",
      "description": "Minimal configuration with basic colors and font"
    }
  ]
}
```

### Execute Endpoints

#### `POST /mako_validate`

Validate a Mako configuration file.

**Request Body:**
```json
{
  "config_path": "/home/user/.config/mako/config"
}
```

**Response:**
```json
{
  "result": {
    "success": true,
    "errors": [],
    "warnings": [],
    "logs": "✓ Config file read successfully\n✓ Config syntax is valid\n"
  }
}
```

#### `POST /mako_apply`

Apply a patch to Mako configuration safely.

**Request Body:**
```json
{
  "config_path": "/home/user/.config/mako/config",
  "patch": "[default]\nfont=monospace 12\nbackground-color=#1e1e2e\n",
  "dry_run": true,
  "backup_path": "/home/user/.config/mako/config.backup"
}
```

**Response:**
```json
{
  "result": {
    "success": true,
    "diff_applied": "+font=monospace 12\n-background-color=#285577\n+background-color=#1e1e2e\n",
    "backup_created": false
  }
}
```

### Health Check

#### `GET /health`

Simple health check endpoint.

**Response:**
```
OK
```

## Workflow

1. **Inspect Configuration Options**: Use `mako_options` to see available options and their types
2. **Generate Template**: Use `mako_templates` to get a starting configuration for your use case
3. **Validate Configuration**: Use `mako_validate` to check your config before applying
4. **Apply Changes**: Use `mako_apply` with `dry_run=true` first to preview changes, then apply with `dry_run=false`
5. **Iterate**: Use validation errors, logs, and warnings to refine your configuration

## Configuration Options

The server supports all standard Mako configuration options including:

- Appearance: `font`, `background-color`, `text-color`, `width`, `height`, `margin`, `padding`, `border-size`, `border-color`, `border-radius`, `progress-color`
- Behavior: `max-visible`, `default-timeout`, `ignore-timeout`, `icons`, `max-icon-size`, `history`
- Layout: `layer`, `anchor`, `sort`, `output`, `group-by`
- Content: `markup`, `actions`

See the full list via the `mako_options` endpoint.

## Safety Features

- **Atomic Writes**: Configuration changes are written atomically to avoid corruption
- **Automatic Backups**: Backups are created before applying changes (unless dry-run)
- **Dry-Run Mode**: Preview changes before applying them
- **Diff Generation**: See exactly what will change before applying
- **Validation**: Syntax and semantic validation before applying changes

## Development

### Project Structure

```
src/
├── main.rs              # Warp server setup and route handlers
├── models/              # Data models (MakoOption, MakoTemplate, etc.)
├── endpoints/           # Endpoint implementations
│   ├── mako_options.rs
│   ├── mako_templates.rs
│   ├── mako_validate.rs
│   └── mako_apply.rs
└── utils/               # Utility modules
    ├── parser.rs        # INI-style config parsing
    ├── file_ops.rs      # File operations with backups
    ├── diff.rs          # Diff generation
    └── logger.rs        # Structured logging
```

### Testing

```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

## License

This project is based on Mako (https://github.com/emersion/mako) and follows similar licensing terms.

