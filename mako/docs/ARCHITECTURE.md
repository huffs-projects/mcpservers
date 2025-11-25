# Mako MCP Server Architecture

## Overview

The Mako MCP Server is a Rust-based implementation of the Model Context Protocol (MCP) server for managing Mako notification daemon configuration files. It communicates via stdio using JSON-RPC 2.0 protocol.

## Module Structure

```
src/
├── main.rs              # Entry point, initializes logging and starts server
├── config.rs            # Server configuration constants
├── models/              # Data models
│   └── mod.rs          # MakoOption, MakoTemplate, ValidationResult, ApplyResult
├── mcp/                 # MCP protocol implementation
│   ├── mod.rs          # Main server loop (stdio I/O)
│   ├── protocol.rs     # JSON-RPC types and error codes
│   ├── handlers.rs     # Request handlers (initialize, tools/list, tools/call)
│   ├── errors.rs       # Error response creation
│   └── tools.rs        # Tool definitions and schemas
├── endpoints/          # Business logic endpoints
│   ├── mako_options.rs    # Query configuration options
│   ├── mako_templates.rs  # Generate config templates
│   ├── mako_validate.rs   # Validate config files
│   └── mako_apply.rs      # Apply config patches
└── utils/               # Utility modules
    ├── parser.rs          # INI-style config parsing
    ├── file_ops.rs        # File operations (read, write, backup)
    ├── diff.rs            # Config diff generation
    ├── logger.rs          # Structured logging
    └── validation.rs      # Value validation (colors, paths, ranges)
```

## Data Flow

### Request Processing

1. **Input**: Server reads JSON-RPC requests from stdin line by line
2. **Parsing**: Each line is parsed into `MCPRequest` structure
3. **Routing**: Request method determines handler:
   - `initialize` → `handlers::handle_initialize()`
   - `tools/list` → `handlers::handle_tools_list()`
   - `tools/call` → `handlers::handle_tools_call()`
4. **Execution**: Handler calls appropriate endpoint function
5. **Response**: Result is serialized to JSON-RPC response format
6. **Output**: Response written to stdout

### Tool Call Flow

```
tools/call request
  ↓
handle_tools_call()
  ↓
Deserialize arguments into typed struct (MakoOptionsArgs, etc.)
  ↓
Call endpoint function (mako_options::get_mako_options(), etc.)
  ↓
Serialize result to JSON
  ↓
Wrap in MCP content format
  ↓
Return MCPResponse
```

## Key Components

### MCP Protocol Module (`src/mcp/`)

- **protocol.rs**: Defines JSON-RPC 2.0 structures (MCPRequest, MCPResponse, MCPError)
- **handlers.rs**: Implements request handlers with typed argument structs
- **errors.rs**: Centralized error response creation
- **tools.rs**: Tool registry with JSON schemas

### Endpoints Module (`src/endpoints/`)

Each endpoint implements a specific Mako configuration operation:

- **mako_options**: Returns list of available configuration options
- **mako_templates**: Generates pre-configured template snippets
- **mako_validate**: Validates config syntax and semantics
- **mako_apply**: Applies patches with validation and backup

### Utils Module (`src/utils/`)

- **parser.rs**: Parses INI-style config files into `ConfigMap` (HashMap structure)
- **file_ops.rs**: Atomic file operations with automatic backups
- **diff.rs**: Generates unified diff between config versions
- **validation.rs**: Validates color formats, paths, integer ranges
- **logger.rs**: Structured logging per endpoint

## Error Handling

### Error Response Flow

1. Errors are caught at appropriate levels
2. Converted to `MCPError` with appropriate JSON-RPC error codes
3. Wrapped in `MCPResponse` with original request `id`
4. Serialized and written to stdout

### Error Codes

- `-32700`: Parse error (invalid JSON)
- `-32600`: Invalid request
- `-32601`: Method not found
- `-32602`: Invalid params
- `-32603`: Internal error
- `-32000`: Server error (validation, apply errors)

## Configuration Management

Constants are centralized in `src/config.rs`:
- Protocol version
- Server name
- Server version (from Cargo.toml via `env!` macro)

## Type Safety

### Typed Arguments

Each tool has a typed argument struct:
- `MakoOptionsArgs { search_term: Option<String> }`
- `MakoTemplatesArgs { use_case: Option<String> }`
- `MakoValidateArgs { config_path: String }`
- `MakoApplyArgs { config_path, patch, dry_run, backup_path }`

This provides compile-time safety and better IDE support compared to generic `Value` types.

## Validation

### Multi-Level Validation

1. **Syntax Validation**: INI parsing ensures valid structure
2. **Type Validation**: Values checked against expected types (integer, boolean, string)
3. **Semantic Validation**: 
   - Color format validation (hex colors)
   - Path validation
   - Range validation (positive integers, non-negative integers)
   - Enum validation (layer, anchor, sort values)
4. **Pre-Apply Validation**: New config validated before writing to prevent broken configs

## Safety Features

### Atomic Writes

Config files are written atomically:
1. Write to temporary file (`.config~`)
2. Atomic rename to final location
3. Prevents corruption if process is interrupted

### Automatic Backups

Before applying changes:
1. Create timestamped backup file
2. Original config preserved
3. Backup path can be customized

### Dry-Run Mode

- Preview changes without applying
- Generate diff output
- Validate new config
- No file modifications

## Testing

### Unit Tests

- Each module has comprehensive unit tests
- Test coverage for:
  - Parser edge cases
  - Diff generation
  - Validation functions
  - Error handling
  - Tool execution

### Integration Tests

End-to-end tests verify:
- MCP protocol compliance
- Tool execution flow
- Error response format
- JSON-RPC 2.0 compliance

## Logging

- All logs go to stderr (MCP requirement)
- Structured logging per endpoint
- Request timing and success/failure tracking
- Error details for debugging

## Future Improvements

- Request batching support
- Config file watching
- Enhanced diff output with context
- Property-based testing
- Performance optimizations

