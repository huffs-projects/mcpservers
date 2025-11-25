# Kitty MCP Server API Documentation

## Overview

The Kitty MCP Server provides tools for querying, validating, and modifying Kitty terminal configuration files. All communication uses the MCP protocol over stdio with JSON-RPC 2.0.

## Protocol

- **Transport**: Stdio (stdin/stdout)
- **Protocol**: JSON-RPC 2.0
- **MCP Version**: 2024-11-05
- **Logging**: Stderr (stdout reserved for JSON-RPC)

## Tools

### kitty_options

Query all known Kitty configuration options.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "search_term": {
      "type": "string",
      "description": "Search term to filter options"
    },
    "category": {
      "type": "string",
      "description": "Filter by category (Fonts, Window, Performance, Layouts, etc.)"
    }
  }
}
```

**Example Request:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "kitty_options",
    "arguments": {
      "search_term": "font",
      "category": "Fonts"
    }
  }
}
```

**Example Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [{
      "type": "text",
      "text": "[{\"name\":\"font_family\",\"type\":\"string\",...}]"
    }]
  }
}
```

### kitty_theming

Return themes, full color palettes, and template snippets.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "theme_name": {
      "type": "string",
      "description": "Filter by specific theme name"
    }
  }
}
```

### kitty_keybindings

Query keybinding actions and modifiers.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "action": {
      "type": "string",
      "description": "Filter by specific action name"
    }
  }
}
```

### kitty_templates

Generate configuration templates.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "category": {
      "type": "string",
      "description": "Filter by template category"
    },
    "use_case": {
      "type": "string",
      "description": "Filter by use case description"
    }
  }
}
```

### kitty_validate

Validate kitty.conf using official syntax rules.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "config_path": {
      "type": "string",
      "description": "Path to kitty.conf file to validate"
    }
  },
  "required": ["config_path"]
}
```

**Example Request:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "kitty_validate",
    "arguments": {
      "config_path": "/Users/username/.config/kitty/kitty.conf"
    }
  }
}
```

**Example Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [{
      "type": "text",
      "text": "{\"success\":true,\"errors\":[],\"warnings\":[],\"logs\":\"Validated 5 options\"}"
    }]
  }
}
```

### kitty_apply

Safely apply patches to kitty.conf with atomic writes and automatic backups.

**Input Schema:**
```json
{
  "type": "object",
  "properties": {
    "config_path": {
      "type": "string",
      "description": "Path to kitty.conf file"
    },
    "patch": {
      "type": "string",
      "description": "Configuration patch to apply"
    },
    "dry_run": {
      "type": "boolean",
      "description": "If true, only show diff without applying changes",
      "default": true
    },
    "backup_path": {
      "type": "string",
      "description": "Optional path for backup file"
    }
  },
  "required": ["config_path", "patch"]
}
```

**Example Request:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "kitty_apply",
    "arguments": {
      "config_path": "/Users/username/.config/kitty/kitty.conf",
      "patch": "font_size 14.0",
      "dry_run": true
    }
  }
}
```

## Error Codes

The server uses standard JSON-RPC 2.0 error codes:

- `-32700`: Parse error (malformed JSON)
- `-32601`: Method not found / Unknown tool
- `-32602`: Invalid params
- `-32603`: Internal error (tool execution failed)
- `-32000`: Server error (file operations, validation, etc.)

## Response Format

All successful tool executions return results in MCP content format:

```json
{
  "jsonrpc": "2.0",
  "id": <request_id>,
  "result": {
    "content": [{
      "type": "text",
      "text": "<JSON string of tool result>"
    }]
  }
}
```

Error responses follow JSON-RPC 2.0 format:

```json
{
  "jsonrpc": "2.0",
  "id": <request_id>,
  "error": {
    "code": <error_code>,
    "message": "<error message>",
    "data": <optional additional data>
  }
}
```

