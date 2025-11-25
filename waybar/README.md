# Waybar Rust MCP Server

Rust-native MCP server for managing Waybar (JSON + CSS) configuration.

## Features

- **Module Inspection**: Query built-in Waybar modules and all configuration options
- **Script Management**: Manage custom script blocks ('custom' and 'exec' modules)
- **Style Management**: Generate and manage CSS rules for Waybar bars, modules, and blocks
- **Template Generation**: Generate full example configs with JSON + CSS snippets for common use-cases
- **Validation**: Validate JSON config and CSS files (syntax, required keys, style correctness)
- **Safe Patching**: Apply patches to JSON and CSS configs safely with atomic writes, diff preview, backup, and dry-run mode

## Installation

```bash
cargo build --release
```

The binary will be located at `target/release/waybar-mcp`.

## Usage

### MCP Server Configuration

Add this to your MCP client configuration (e.g., Cursor, Claude Desktop):

```json
{
  "mcpServers": {
    "waybar": {
      "command": "/path/to/waybar-mcp",
      "args": []
    }
  }
}
```

### Available Tools

#### `waybar_modules`

List built-in Waybar modules and all configuration options.

**Parameters:**
- `filter_module` (optional): Module name to filter by

**Example:**
```json
{
  "name": "waybar_modules",
  "arguments": {
    "filter_module": "battery"
  }
}
```

#### `waybar_scripts`

Inspect custom script blocks from a Waybar config file.

**Parameters:**
- `config_path` (optional): Path to Waybar config file
- `filter_name` (optional): Script name to filter by

**Example:**
```json
{
  "name": "waybar_scripts",
  "arguments": {
    "config_path": "~/.config/waybar/config",
    "filter_name": "custom"
  }
}
```

#### `waybar_style`

Return CSS style rules for bars, modules, blocks, and fonts.

**Parameters:**
- `selector` (optional): CSS selector to filter by

**Example:**
```json
{
  "name": "waybar_style",
  "arguments": {
    "selector": "#battery"
  }
}
```

#### `waybar_templates`

Generate Waybar JSON + CSS templates for common use-cases.

**Parameters:**
- `use_case` (optional): Use case name (e.g., 'hyprland-default', 'battery', 'network', 'cpu')

**Example:**
```json
{
  "name": "waybar_templates",
  "arguments": {
    "use_case": "hyprland-default"
  }
}
```

#### `waybar_validate`

Validate Waybar JSON + CSS files.

**Parameters:**
- `config_path` (required): Path to Waybar JSON config file
- `css_path` (optional): Path to CSS file

**Example:**
```json
{
  "name": "waybar_validate",
  "arguments": {
    "config_path": "~/.config/waybar/config",
    "css_path": "~/.config/waybar/style.css"
  }
}
```

#### `waybar_apply`

Apply patches to JSON and CSS safely with backup and dry-run support.

**Parameters:**
- `config_path` (required): Path to Waybar JSON config file
- `css_path` (optional): Path to CSS file
- `patch_json` (required): JSON patch to apply (can be object for merge or array for RFC 6902 patch)
- `patch_css` (optional): CSS patch to apply
- `dry_run` (optional, default: true): If true, show diff without applying
- `backup_path` (optional): Directory for backups

**Example:**
```json
{
  "name": "waybar_apply",
  "arguments": {
    "config_path": "~/.config/waybar/config",
    "patch_json": "{\"battery\": {\"format\": \"{capacity}%\"}}",
    "dry_run": true
  }
}
```

## Supported Modules

The server includes schema definitions for the following built-in Waybar modules:

- `battery` - Battery status and charging information
- `cpu` - CPU usage monitoring
- `memory` - Memory usage monitoring
- `network` - Network interface status (WiFi/Ethernet)
- `clock` - Date and time display
- `tray` - System tray icons
- `custom` - Custom script execution (repeating)
- `exec` - One-time script execution
- `idle_inhibitor` - Idle inhibitor control
- `pulseaudio` - Audio volume control
- `backlight` - Screen brightness control
- `disk` - Disk usage monitoring
- `temperature` - CPU/system temperature
- `window` - Active window title (Sway/Wayland)
- `workspaces` - Workspace switcher (Sway/Wayland)
- `mpd` - Music Player Daemon integration
- `bluetooth` - Bluetooth device status

## Documentation References

All module options and features reference official Waybar documentation:

- [Waybar GitHub](https://github.com/Alexays/Waybar)
- [Waybar Wiki Examples](https://github.com/Alexays/Waybar/wiki/Examples)
- [Hyprland Status Bars](https://wiki.hypr.land/Useful-Utilities/Status-Bars/)
- [Built-in Modules](https://waybar.org/what-modules-come-built-in-with-waybar/)
- [Custom Scripts](https://waybar.org/can-i-add-custom-scripts-to-waybar/)
- [Styling/CSS](https://waybar.org/how-can-i-style-waybar-with-css/)
- [Config Location](https://waybar.org/where-is-waybars-configuration-file-located/)

## Development

### Project Structure

```
src/
├── main.rs              # MCP server entry point
├── models/              # Data models
│   ├── module_option.rs
│   ├── script.rs
│   ├── style_snippet.rs
│   ├── template.rs
│   ├── validation_result.rs
│   └── apply_result.rs
├── endpoints/           # MCP tool handlers
│   ├── waybar_modules.rs
│   ├── waybar_scripts.rs
│   ├── waybar_style.rs
│   ├── waybar_templates.rs
│   ├── waybar_validate.rs
│   └── waybar_apply.rs
└── utils/               # Utility modules
    ├── parser.rs
    ├── schema.rs
    ├── file_ops.rs
    ├── diff.rs
    ├── logger.rs
    └── doc_mapper.rs
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

## License

This project is provided as-is for managing Waybar configurations via the MCP protocol.

