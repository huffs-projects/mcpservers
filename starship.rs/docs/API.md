# Starship MCP Server API Documentation

## Overview

The Starship MCP Server provides a JSON-RPC-like API for querying and managing Starship prompt configurations. All endpoints accept POST requests to `/mcp` with a JSON body containing the method name and parameters.

## Base URL

```
http://localhost:8080
```

(Default port is 8080, configurable via `PORT` environment variable)

## Request Format

All requests follow this format:

```json
{
  "method": "method_name",
  "params": {
    // method-specific parameters
  }
}
```

## Response Format

All responses follow this format:

```json
{
  "result": {
    // method-specific result data
  },
  "error": null
}
```

Or in case of error:

```json
{
  "result": null,
  "error": {
    "code": -32603,
    "message": "Error description"
  }
}
```

## Error Codes

- `-32601`: Method not found
- `-32602`: Invalid params
- `-32603`: Internal error

## Endpoints

### starship_options

Query Starship configuration options.

**Parameters:**
- `search_term` (optional, string): Filter options by name or description
- `category` (optional, string): Filter options by category (e.g., "general", "module")

**Example Request:**
```json
{
  "method": "starship_options",
  "params": {
    "search_term": "git",
    "category": "module"
  }
}
```

**Example Response:**
```json
{
  "result": [
    {
      "name": "git_branch.format",
      "type": "string",
      "default": "$symbol$branch",
      "category": "module",
      "description": "The format string for the git_branch module",
      "example": "$symbol$branch",
      "documentation_url": "https://starship.rs/config/#git-branch"
    }
  ],
  "error": null
}
```

### starship_presets

Query available Starship presets.

**Parameters:**
- `preset_name` (optional, string): Filter by specific preset name

**Example Request:**
```json
{
  "method": "starship_presets",
  "params": {
    "preset_name": "no-runtime-versions"
  }
}
```

**Example Response:**
```json
{
  "result": [
    {
      "preset_name": "no-runtime-versions",
      "snippet": "[git_branch]\nformat = \"$symbol$branch\"\n\n[nodejs]\ndisabled = true",
      "description": "Hide runtime version information",
      "documentation_url": "https://starship.rs/presets/#no-runtime-versions"
    }
  ],
  "error": null
}
```

### starship_templates

Generate Starship configuration templates.

**Parameters:**
- `category` (optional, string): Filter templates by category
- `use_case` (optional, string): Filter templates by use case

**Example Request:**
```json
{
  "method": "starship_templates",
  "params": {
    "category": "minimal"
  }
}
```

### starship_validate

Validate a Starship configuration file.

**Parameters:**
- `config_path` (string, required): Path to the configuration file to validate

**Example Request:**
```json
{
  "method": "starship_validate",
  "params": {
    "config_path": "/home/user/.config/starship.toml"
  }
}
```

**Example Response:**
```json
{
  "result": {
    "success": true,
    "errors": [],
    "warnings": ["No 'format' field specified - using default"],
    "logs": "✓ TOML syntax is valid\n✓ Structure validation passed\n"
  },
  "error": null
}
```

### starship_apply

Apply configuration changes to a Starship config file.

**Parameters:**
- `config_path` (string, required): Path to the configuration file
- `patch` (string, required): TOML patch to apply
- `dry_run` (boolean, optional, default: true): If true, don't actually apply changes
- `backup_path` (string, optional): Custom path for backup file

**Example Request:**
```json
{
  "method": "starship_apply",
  "params": {
    "config_path": "/home/user/.config/starship.toml",
    "patch": "[git_branch]\nformat = \"$branch\"",
    "dry_run": false
  }
}
```

**Example Response:**
```json
{
  "result": {
    "success": true,
    "diff_applied": "-format = \"$all\"\n+[git_branch]\n+format = \"$branch\"",
    "backup_created": true
  },
  "error": null
}
```

## Health Check

A simple health check endpoint is available at `/health`:

```bash
curl http://localhost:8080/health
```

**Response:**
```json
{
  "status": "ok"
}
```

## Security

- All file paths are validated to prevent directory traversal attacks
- Input parameters are validated for length and format
- CORS can be configured via `CORS_ALLOWED_ORIGINS` environment variable

## Configuration

See the main README for environment variable configuration options.

