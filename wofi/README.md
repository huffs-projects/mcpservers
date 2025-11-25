# Wofi Rust MCP Server

Rust-native MCP server for managing Wofi launcher configuration, styles, modes, and dmenu-compatible scripts. Fully informed by ALL authoritative documentation:

- **Official Wofi project page**: https://sr.ht/~scoopta/wofi/
- **Official wofi man pages**: https://github.com/SimplyCEO/wofi/man
- **CloudNinja Wofi Deep Documentation**: https://cloudninja.pw/docs/wofi.html

## Features

- **Complete Introspection**: Query all Wofi options, modes, styles, and templates
- **Validation**: Comprehensive validation of config files and CSS
- **Safe Patching**: Atomic writes with rollback, backups, and diff previews
- **Style Management**: Extract and validate CSS selectors
- **Mode Management**: Parse builtin and custom modes
- **Example Generation**: Pre-built templates for common setups
- **Documentation Linking**: Link any config key, mode, or selector to authoritative docs

## Installation

```bash
cargo build --release
```

## Usage

The MCP server communicates via stdio using JSON-RPC 2.0 protocol.

### Available Endpoints

#### `wofi_config_locations`
Returns Wofi config search paths in priority order:
- `$XDG_CONFIG_HOME/wofi/config`
- `~/.config/wofi/config`
- `/etc/xdg/wofi/config`
- `/usr/share/wofi/config` (fallback)

#### `wofi_options`
Get all Wofi runtime options with optional filtering.

**Parameters:**
- `filter` (optional): Filter options by name, description, or type

**Returns:** List of `WofiOption` objects

#### `wofi_templates`
Get configuration templates for common setups.

**Parameters:**
- `use_case` (optional): Filter templates by use case

**Returns:** List of `WofiTemplate` objects including:
- Minimal launcher
- App launcher (drun)
- Command launcher (run)
- SSH launcher
- Fuzzy fullscreen mode
- Hyprland-optimized launcher
- Dmenu compatibility mode
- Custom script mode

#### `wofi_styles`
Get CSS style rules and selectors.

**Parameters:**
- `selector` (optional): Filter by CSS selector

**Returns:** List of `WofiStyleRule` objects

#### `wofi_modes`
Get available Wofi modes (builtin and custom).

**Parameters:**
- `filter` (optional): Filter modes by name or type

**Returns:** List of `WofiMode` objects

#### `wofi_validate`
Validate Wofi config and CSS files.

**Parameters:**
- `config_path`: Path to config file
- `css_path` (optional): Path to CSS file

**Returns:** `ValidationResult` with errors, warnings, and invalid items

#### `wofi_apply`
Apply patches to config and CSS files with atomic writes.

**Parameters:**
- `config_path`: Path to config file
- `css_path` (optional): Path to CSS file
- `patch_config`: New config content
- `patch_css` (optional): New CSS content
- `dry_run` (optional, default: true): If true, only show diff without applying

**Returns:** `ApplyResult` with diff and backup path

#### `wofi_docs`
Get documentation links for a keyword.

**Parameters:**
- `keyword`: Configuration key, mode, or selector name

**Returns:** Documentation string with links to sr.ht, man pages, and CloudNinja docs

## Project Structure

```
src/
├── main.rs                 # MCP server entry point
├── models/                 # Data structures
│   ├── wofi_option.rs
│   ├── wofi_template.rs
│   ├── wofi_style_rule.rs
│   ├── wofi_mode.rs
│   ├── validation_result.rs
│   └── apply_result.rs
├── modules/                # Core business logic
│   ├── wofi_config_locations.rs
│   ├── wofi_options.rs
│   ├── wofi_templates.rs
│   ├── wofi_styles.rs
│   ├── wofi_modes.rs
│   ├── wofi_validate.rs
│   ├── wofi_apply.rs
│   └── wofi_docs.rs
└── utils/                  # Utility functions
    ├── config_locator.rs
    ├── config_parser.rs
    ├── css_parser.rs
    ├── mode_parser.rs
    ├── doc_mapper.rs
    ├── diff_utils.rs
    └── atomic_write.rs
```

## Guidelines

- All behavior matches sr.ht canonical definitions first, then manpages, then CloudNinja explanations
- Apply system remains fully atomic with rollback capability
- CSS selectors validated against actual wofi GTK widget hierarchy
- Modes respect stdin/stdout formatting

## Development

```bash
# Run tests
cargo test

# Build release
cargo build --release

# Check for issues
cargo clippy
```

## License

This project is provided as-is for managing Wofi configurations.

