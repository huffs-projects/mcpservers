# Zsh Rust MCP Server

A Rust-native MCP (Model Context Protocol) server to manage Zsh configuration declaratively.

## Overview

This MCP server provides tools for:
- Querying Zsh shell options, builtins, and modules
- Generating snippet templates for common Zsh configurations
- Validating existing `.zshrc` or Zsh config files
- Applying safe patches to Zsh configuration files

Built with authoritative Zsh sources:
- [Zsh Reference Manual](https://zsh.sourceforge.io/Doc/Release/zsh_toc.html)
- [Zsh Guide](https://zsh.sourceforge.io/Guide/zshguide.html)

## Features

### 1. Query Zsh Options (`zsh_options`)

List Zsh shell options with metadata including:
- Option name and scope (GLOBAL, BOURNEOPT, SH_GLOB, etc.)
- Type (boolean, string, integer, function)
- Default value
- Description
- Documentation URL

**Example MCP Request:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "zsh_options",
    "arguments": {
      "search_term": "history",
      "scope": "GLOBAL"
    }
  }
}
```

### 2. Generate Templates (`zsh_templates`)

Generate snippet templates for:
- Prompts (powerline, minimal)
- Completion systems (basic, advanced)
- Vi mode key bindings
- History configuration
- Module loading

**Example MCP Request:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "zsh_templates",
    "arguments": {
      "use_case": "completion"
    }
  }
}
```

### 3. Validate Config (`zsh_validate`)

Validate Zsh config files for:
- Syntactic correctness
- Unrecognized options
- Common misconfigurations
- Suspicious patterns (e.g., `$_` usage)

**Example MCP Request:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "zsh_validate",
    "arguments": {
      "config_path": "~/.zshrc"
    }
  }
}
```

### 4. Apply Patches (`zsh_apply`)

Safely apply configuration changes with:
- Dry-run mode (default)
- Automatic backup creation
- Unified diff preview
- Atomic writes

**Example MCP Request:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "zsh_apply",
    "arguments": {
      "config_path": "~/.zshrc",
      "patch": "--- original\n+++ modified\n+setopt EXTENDED_HISTORY\n",
      "dry_run": true,
      "backup_path": "~/.zshrc.backups"
    }
  }
}
```

## Installation

### Prerequisites

- Rust 1.70+ (with edition 2021)
- Cargo

### Build

```bash
cargo build --release
```

### Run

The server communicates via stdio using the MCP (Model Context Protocol) over JSON-RPC 2.0.

```bash
# Build and run
cargo build --release
./target/release/zsh-mcp-server

# Or run directly
cargo run
```

The server reads JSON-RPC requests from stdin and writes responses to stdout. It's designed to be launched as a subprocess by MCP clients like Cursor.

## MCP Integration

### Cursor Configuration

Add to your Cursor MCP settings:

```json
{
  "mcpServers": {
    "zsh": {
      "command": "/path/to/zsh-mcp-server/target/release/zsh-mcp-server"
    }
  }
}
```

## Available Tools

### `zsh_options`

List Zsh shell options with metadata.

**Arguments:**
- `search_term` (optional): Filter by option name or description
- `scope` (optional): Filter by option scope (e.g., "GLOBAL", "BOURNEOPT")

**Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [{
      "type": "text",
      "text": "[{\"name\":\"EXTENDED_HISTORY\",\"scope\":\"GLOBAL\",\"type\":\"boolean\",...}]"
    }]
  }
}
```

### `zsh_templates`

Generate snippet templates for Zsh configuration.

**Arguments:**
- `use_case` (optional): Filter by use case (prompt, completion, vi-mode, keybindings, history, modules)

### `zsh_validate`

Validate a Zsh configuration file for syntactic correctness and common misconfigurations.

**Arguments:**
- `config_path` (required): Path to Zsh config file (supports `~` and `$HOME` expansion)

**Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [{
      "type": "text",
      "text": "{\"success\":true,\"errors\":[],\"warnings\":[],\"logs\":\"...\"}"
    }]
  }
}
```

### `zsh_apply`

Apply configuration changes to Zsh config safely.

**Arguments:**
- `config_path` (required): Path to Zsh config file
- `patch` (required): Unified diff or structured patch content
- `dry_run` (optional, default: true): Perform dry-run without applying
- `backup_path` (optional): Custom backup directory

**Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [{
      "type": "text",
      "text": "{\"success\":true,\"diff_applied\":\"...\",\"backup_created\":true}"
    }]
  }
}
```

## Project Structure

```
src/
├── main.rs              # Entry point with async main
├── lib.rs               # Library root (for tests)
├── mcp.rs               # MCP stdio server implementation
├── error.rs             # Custom error types
├── models/              # Data models
│   └── mod.rs
├── endpoints/           # Tool implementations
│   ├── zsh_options.rs
│   ├── zsh_templates.rs
│   ├── zsh_validate.rs
│   └── zsh_apply.rs
└── utils/               # Utility modules
    ├── parser.rs        # Zsh config parsing
    ├── schema.rs        # Zsh options schema
    ├── file_ops.rs      # File operations with path expansion
    ├── diff.rs          # Diff computation
    └── logger.rs        # Tracing-based logging
```

## Testing

Run the test suite:

```bash
cargo test
```

Tests cover:
- MCP protocol handling
- Tool execution
- Config parsing and validation
- File operations and path expansion

## Features

- **Async I/O**: Non-blocking stdio communication for better performance
- **Response Caching**: Cached responses for `tools/list` and `initialize` methods
- **Path Expansion**: Automatic `~` and `$HOME` expansion in config paths
- **Error Handling**: Comprehensive error types with proper JSON-RPC error codes
- **Structured Logging**: Integrated with tracing framework for observability
- **Input Validation**: Request and argument validation before processing

## Guidelines

- Always dry-run patching before making persistent changes
- Backups are created automatically before applying patches
- The server treats `.zshrc` as the primary config file but supports other patterns
- All option metadata is derived from the official Zsh reference manual
- Paths support `~` and `$HOME` expansion for convenience

## Troubleshooting

### Server not responding

- Ensure the server binary is executable
- Check that stdin/stdout are not redirected incorrectly
- Verify JSON-RPC request format matches specification

### Path expansion issues

- Ensure `$HOME` environment variable is set
- Use absolute paths if `~` expansion fails
- Check file permissions for config files

### Error codes

- `-32700`: Parse error (invalid JSON)
- `-32600`: Invalid Request (malformed request structure)
- `-32601`: Method not found
- `-32602`: Invalid params (missing or incorrect parameters)
- `-32603`: Internal error (server-side error)

## License

This project is provided as-is for managing Zsh configurations.

