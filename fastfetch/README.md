# Fastfetch MCP Server

An MCP (Model Context Protocol) server for configuring fastfetch, written in Rust.

## Overview

This MCP server provides tools to help configure fastfetch, including:
- Reading and parsing fastfetch config files (JSONC format)
- Writing/modifying config files
- Validating configs against JSON schema
- Listing available modules and logos
- Generating new config files (minimal or full)
- Providing help with format strings and color specifications

## Project Structure

```
fastfetch/
├── Cargo.toml          # Project dependencies
├── README.md           # This file
├── src/
│   ├── main.rs         # MCP server entry point
│   ├── config.rs       # Config file reading/writing
│   ├── schema.rs       # JSON schema validation
│   ├── modules.rs      # Module and logo listing
│   └── tools.rs        # MCP tool implementations
└── schemas/            # JSON schema files (if needed)
```

## Dependencies

- `rmcp` - Official Rust MCP SDK (Model Context Protocol server framework)
- `serde` / `serde_json` - JSON serialization
- `jsonc-parser` - JSONC parsing (JSON with comments)
- `jsonschema` - JSON schema validation
- `tokio` - Async runtime
- `anyhow` - Error handling
- `dirs` - Finding config directory
- `reqwest` - HTTP client for fetching schema
- `thiserror` - Custom error types

## Implementation Status

### Completed
- ✅ Config file handling (read/write JSONC)
- ✅ Schema validation
- ✅ Module and logo listing
- ✅ All MCP tool implementations
- ✅ Project structure and dependencies
- ✅ MCP server wiring - Using official `rmcp` SDK

## MCP Tools

The server exposes the following tools:

1. **read_fastfetch_config** - Read and parse a fastfetch configuration file
   - Optional parameter: `path` (string) - Path to config file

2. **write_fastfetch_config** - Write a fastfetch configuration to file
   - Required parameter: `config` (object) - The configuration object
   - Optional parameter: `path` (string) - Path to config file

3. **validate_fastfetch_config** - Validate a config against JSON schema
   - Optional parameter: `config` (object) - Config to validate
   - Optional parameter: `path` (string) - Path to config file

4. **list_fastfetch_modules** - List all available fastfetch modules

5. **list_fastfetch_logos** - List all available fastfetch logos

6. **generate_fastfetch_config** - Generate a new config file
   - Optional parameter: `full` (boolean) - Generate full config
   - Optional parameter: `path` (string) - Path to write config

7. **fastfetch_format_help** - Get help with format strings and colors

## Configuration File Location

By default, the server looks for fastfetch config files at:
- `~/.config/fastfetch/config.jsonc`

You can override this by providing a `path` parameter to the tools.

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run
```

The server communicates via stdio using JSON-RPC 2.0 protocol, implemented using the official `rmcp` SDK.

## Implementation Details

The server uses the official `rmcp` SDK with the following features:
- `ServerHandler` trait implementation for all MCP protocol methods
- `stdio` transport for JSON-RPC 2.0 communication
- Full support for tools, resources, and prompts
- Proper error handling and protocol compliance

Refer to the [official rmcp documentation](https://github.com/modelcontextprotocol/rust-sdk) for more information.

## Fastfetch Resources

- [Configuration Guide](https://github.com/fastfetch-cli/fastfetch/wiki/Configuration)
- [JSON Schema](https://github.com/fastfetch-cli/fastfetch/wiki/Json-Schema-root)
- [Format String Guide](https://github.com/fastfetch-cli/fastfetch/wiki/Format-String-Guide)
- [Color Format Specification](https://github.com/fastfetch-cli/fastfetch/wiki/Color-Format-Specification)
