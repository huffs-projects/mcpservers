# NeoMutt MCP Server

A Model Context Protocol (MCP) server that helps configure NeoMutt by providing documentation lookup, configuration generation, validation, and interactive assistance.

## Features

- **Documentation Lookup**: Search NeoMutt documentation and get details about configuration options
- **Configuration Generation**: Generate muttrc files based on your requirements
- **Configuration Validation**: Validate and lint your NeoMutt configuration files
- **Interactive Assistant**: Guided setup wizard and troubleshooting help

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Build

```bash
cargo build --release
```

The binary will be at `target/release/neomutt-mcp-server`.

## Usage

The MCP server communicates via JSON-RPC 2.0 over stdio. It's designed to be used with MCP-compatible clients.

### Running the Server

```bash
./target/release/neomutt-mcp-server
```

The server reads JSON-RPC requests from stdin and writes responses to stdout.

## Available Tools

### Documentation Tools

#### `search_docs`
Search NeoMutt documentation.

**Parameters:**
- `query` (string, required): Search query

**Example:**
```json
{
  "method": "tools/call",
  "params": {
    "name": "search_docs",
    "arguments": {
      "query": "imap"
    }
  }
}
```

#### `get_config_option`
Get details about a specific NeoMutt configuration option.

**Parameters:**
- `option` (string, required): Configuration option name

**Example:**
```json
{
  "method": "tools/call",
  "params": {
    "name": "get_config_option",
    "arguments": {
      "option": "imap_user"
    }
  }
}
```

#### `get_guide_section`
Retrieve a specific guide section from neomutt.org.

**Parameters:**
- `section` (string, required): Guide section name or URL

**Example:**
```json
{
  "method": "tools/call",
  "params": {
    "name": "get_guide_section",
    "arguments": {
      "section": "configuration"
    }
  }
}
```

### Configuration Generation

#### `generate_config`
Generate a NeoMutt configuration file based on requirements.

**Parameters:**
- `requirements` (string, required): Description of configuration requirements

**Example:**
```json
{
  "method": "tools/call",
  "params": {
    "name": "generate_config",
    "arguments": {
      "requirements": "IMAP and SMTP with SSL, Maildir format"
    }
  }
}
```

#### `add_account`
Add an email account configuration to a muttrc file.

**Parameters:**
- `email` (string, required): Email address
- `imap_server` (string, required): IMAP server hostname
- `smtp_server` (string, required): SMTP server hostname
- `imap_port` (number, optional): IMAP port (default: 993)
- `smtp_port` (number, optional): SMTP port (default: 587)
- `use_ssl` (boolean, optional): Use SSL/TLS (default: true)

**Example:**
```json
{
  "method": "tools/call",
  "params": {
    "name": "add_account",
    "arguments": {
      "email": "user@example.com",
      "imap_server": "imap.example.com",
      "smtp_server": "smtp.example.com",
      "imap_port": 993,
      "smtp_port": 587,
      "use_ssl": true
    }
  }
}
```

### Configuration Validation

#### `validate_config`
Validate a NeoMutt configuration file.

**Parameters:**
- `config` (string, required): Configuration file content or path

**Example:**
```json
{
  "method": "tools/call",
  "params": {
    "name": "validate_config",
    "arguments": {
      "config": "set real_name = \"John Doe\"\nset from = \"john@example.com\""
    }
  }
}
```

#### `check_options`
Verify option names and values in a configuration.

**Parameters:**
- `config` (string, required): Configuration file content

**Example:**
```json
{
  "method": "tools/call",
  "params": {
    "name": "check_options",
    "arguments": {
      "config": "set real_name = \"John Doe\""
    }
  }
}
```

#### `lint_config`
Find common mistakes and suggest fixes in a configuration.

**Parameters:**
- `config` (string, required): Configuration file content

**Example:**
```json
{
  "method": "tools/call",
  "params": {
    "name": "lint_config",
    "arguments": {
      "config": "set realname = John Doe"
    }
  }
}
```

### Interactive Assistant

#### `setup_wizard`
Guided setup process for NeoMutt configuration.

**Parameters:**
- `step` (string, optional): Current step in the wizard (default: "start")

**Example:**
```json
{
  "method": "tools/call",
  "params": {
    "name": "setup_wizard",
    "arguments": {
      "step": "basic_info"
    }
  }
}
```

#### `suggest_config`
Suggest configurations based on use case.

**Parameters:**
- `use_case` (string, required): Description of the use case

**Example:**
```json
{
  "method": "tools/call",
  "params": {
    "name": "suggest_config",
    "arguments": {
      "use_case": "Gmail with encryption"
    }
  }
}
```

#### `troubleshoot`
Help diagnose configuration issues.

**Parameters:**
- `error` (string, required): Error message or issue description
- `config` (string, optional): Configuration file content

**Example:**
```json
{
  "method": "tools/call",
  "params": {
    "name": "troubleshoot",
    "arguments": {
      "error": "Connection refused",
      "config": "set folder = \"imap://imap.example.com:993\""
    }
  }
}
```

## Sample Configurations

The `data/samples/` directory contains example configurations:

- `basic.muttrc` - Minimal configuration to get started
- `maildir.muttrc` - Maildir format configuration
- `multiple-accounts.muttrc` - Multiple accounts using account-hook
- `gmail.muttrc` - Gmail-specific configuration
- `encryption.muttrc` - GPG/PGP encryption configuration

## Project Structure

```
neomutt-mcp-server/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs              # MCP server entry point
│   ├── handlers/            # Tool handlers
│   │   ├── docs.rs          # Documentation lookup
│   │   ├── config_gen.rs    # Configuration generation
│   │   ├── config_validate.rs # Configuration validation
│   │   └── interactive.rs    # Interactive assistant
│   ├── models/              # Data structures
│   │   └── config.rs        # Configuration models
│   └── parser/              # Configuration parsing
│       └── muttrc.rs        # muttrc parser
└── data/
    ├── docs/                # Cached documentation
    └── samples/             # Sample configurations
```

## Development

### Running Tests

```bash
cargo test
```

### Building

```bash
cargo build
```

### Running in Debug Mode

```bash
cargo run
```

## Resources

- [NeoMutt Configuration Guide](https://neomutt.org/guide/configuration.html)
- [NeoMutt Samples Repository](https://github.com/neomutt/samples)
- [NeoMutt Manual Pages](https://neomutt.org/man)
- [NeoMutt Guide](https://neomutt.org/guide/)

## License

This project is provided as-is for assisting with NeoMutt configuration.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

