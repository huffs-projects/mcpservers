# Neovim Ultra-Detailed Rust MCP Server

A fully-described, deeply modular Rust MCP server that provides an agent with complete programmatic control over Neovim configuration. This includes introspection of builtin Neovim options, discovery of LazyVim conventions, generation of idiomatic config structures, Lua AST-based parsing and patching, syntax validation, plugin graph inspection, and safe incremental configuration application workflows.

## Features

- **Neovim Option Introspection**: Full database of Neovim options with metadata, documentation, and validation
- **LazyVim Support**: Deep integration with LazyVim conventions and plugin structure
- **Lua AST Parsing**: Tree-sitter based Lua AST parsing for syntax validation and code transformation
- **Template Generation**: Generate idiomatic Neovim config snippets following LazyVim patterns
- **Multi-stage Validation**: Syntax, semantic, plugin dependency, and runtime path validation
- **Safe Configuration Application**: Atomic file writes with backup and rollback support
- **Plugin Graph Analysis**: Detect cycles, resolve dependencies, and determine load order

## Architecture

### Core Subsystem (`src/core/`)
- `ast.rs` - Lua AST parser based on tree-sitter-lua
- `diagnostics.rs` - LSP-like diagnostic model for config errors
- `model.rs` - Strongly typed Neovim config entities
- `runtime.rs` - Neovim runtime resolution and doc indexing
- `patch.rs` - Lua AST transformer and diff generator
- `template.rs` - Snippet/template provider framework
- `schema.rs` - Typed schema for options derived from documentation
- `nvinfo.rs` - Integration with Neovim's `api_info()`

### Plugins Subsystem (`src/plugins/`)
- `lazyvim.rs` - Model LazyVim plugin structure & conventions
- `registry.rs` - Plugin registry, dependencies, event triggers
- `plugin_graph.rs` - Directed graph representing plugin load order

### Endpoints Subsystem (`src/endpoints/`)
- `options.rs` - Implements `nvim_options` query endpoint
- `templates.rs` - Implements LazyVim-aware snippet generation
- `validate.rs` - Executes full validation pipeline
- `apply.rs` - Safe file mutation with rollback
- `discover.rs` - Identify config roots (init.lua, lua/, plugin/)

### Utils Subsystem (`src/utils/`)
- `fs.rs` - Atomic writes, backups, cross-platform path handling
- `diff.rs` - Unified diff + AST-aware diff modes
- `logger.rs` - Structured JSON logs for agent reasoning
- `lua_printer.rs` - Pretty-print Lua AST back to code

## API Endpoints

### `GET /nvim_options`
Returns a full database of Neovim option definitions.

**Query Parameters:**
- `search` (optional): Search options by name or description
- `scope` (optional): Filter by scope (global, window, buffer)

**Response:** Array of `NvimOption` objects

### `POST /nvim_templates`
Generate idiomatic Neovim config snippets.

**Body:**
```json
{
  "use_case": "lazyvim_keymap",
  "parameters": {
    "key": "<leader>xx",
    "desc": "Description"
  }
}
```

**Response:** Array of `NvimTemplate` objects

### `POST /nvim_validate`
Perform multi-stage validation of Neovim configuration.

**Body:**
```json
{
  "config_roots": ["~/.config/nvim"]
}
```

**Response:** `ValidationResult` with syntax errors, semantic errors, warnings, etc.

### `POST /nvim_apply`
Apply safe patches to Neovim config files.

**Body:**
```json
{
  "file_path": "~/.config/nvim/init.lua",
  "patch": "--- original\n+++ modified\n...",
  "dry_run": true
}
```

**Response:** `ApplyResult` with success status, diff, and backup path

### `GET /nvim_discover`
Detect Neovim config root using XDG paths or ~/.config/nvim.

**Response:** Array of discovered config paths

### `GET /health`
Health check endpoint.

## Usage

### Building

```bash
cargo build --release
```

### Running

```bash
cargo run
```

The server will start on `http://127.0.0.1:3030`

### Example: Query Neovim Options

```bash
curl "http://127.0.0.1:3030/nvim_options?search=tabstop"
```

### Example: Generate Template

```bash
curl -X POST http://127.0.0.1:3030/nvim_templates \
  -H "Content-Type: application/json" \
  -d '{"use_case": "lazyvim_keymap", "parameters": {"key": "<leader>xx"}}'
```

### Example: Validate Configuration

```bash
curl -X POST http://127.0.0.1:3030/nvim_validate \
  -H "Content-Type: application/json" \
  -d '{"config_roots": ["~/.config/nvim"]}'
```

## Workflow

1. **Discover** Neovim config structure using `nvim_discover`
2. **Query** Neovim option metadata using `nvim_options`
3. **Generate** config templates following LazyVim conventions using `nvim_templates`
4. **Validate** configuration using `nvim_validate`
5. **Apply** safe changes via AST/diff patching using `nvim_apply`
6. **Repeat** until no warnings/errors remain

## Guidelines

- Prefer AST-level patching for Lua files to avoid breaking formatting or syntax
- All generated templates follow LazyVim standard directory layout
- Detailed logs are provided for reasoning about plugin graphs and dependencies
- Validate before every apply operation
- Always generate backups before writing

## Dependencies

- **warp** - Web framework
- **tokio** - Async runtime
- **serde/serde_json** - Serialization
- **tree-sitter/tree-sitter-lua** - Lua AST parsing
- **walkdir** - Directory traversal
- **regex** - Pattern matching
- **tracing** - Structured logging

## References

- [Neovim Runtime Documentation](https://neovim.io/doc/)
- [LazyVim Configuration Reference](https://www.lazyvim.org/)

## License

This project is part of the MCP (Model Context Protocol) ecosystem.

