# Nix Flakes MCP Server

A Rust-native MCP (Model Context Protocol) server providing structured access to Nix Flakes, grounded in authoritative sources from nix.dev and the nixos-and-flakes-book repository.

## Version

rust-2.0

## Features

This MCP server exposes five main endpoints for working with Nix Flakes:

1. **flake_inputs** - Query flake inputs and their canonical sources
2. **flake_outputs** - Query outputs and attributes using official flake conventions
3. **flake_eval** - Evaluate arbitrary flake expressions safely
4. **flake_build** - Build flake outputs with dry-run and logging
5. **flake_scaffold** - Scaffold new flake projects, generate flake.nix files from templates, or add outputs to existing flakes

## Requirements

- Rust 1.70+ (edition 2021)
- Nix with Flakes support
- `nix` command available in PATH

## Building

```bash
cargo build --release
```

## Running

The server runs on port 8080 by default, or you can set the `PORT` environment variable:

```bash
PORT=3000 cargo run
```

## API Endpoints

### MCP Protocol Endpoint

**POST /mcp**

Handles MCP protocol requests:
- `tools/list` - List available tools
- `tools/call` - Call a specific tool

### Direct HTTP Endpoints

**POST /flake_inputs**

List all inputs of a flake.

Request:
```json
{
  "flake_path": "github:nixos/nixpkgs",
  "filter": "nixpkgs"  // optional
}
```

Response:
```json
{
  "inputs": [
    {
      "name": "nixpkgs",
      "url": "github:NixOS/nixpkgs",
      "revision": "abc123...",
      "type": "git",
      "documentation_url": "https://nixos.org/manual/nix/stable/command-ref/new-cli/nix3-flake.html#flake-inputs"
    }
  ]
}
```

**POST /flake_outputs**

List outputs of a flake.

Request:
```json
{
  "flake_path": "github:nixos/nixpkgs",
  "filter": "packages"  // optional
}
```

Response:
```json
{
  "outputs": [
    {
      "attribute": "packages.x86_64-linux.hello",
      "drv_path": "/nix/store/...",
      "type": "package",
      "documentation_url": "https://nixos.org/manual/nix/stable/command-ref/new-cli/nix3-flake.html#flake-outputs"
    }
  ]
}
```

**POST /flake_eval**

Evaluate a flake expression.

Request:
```json
{
  "flake_path": "github:nixos/nixpkgs",
  "expression": "lib.version",
  "json_output": true  // optional, default: true
}
```

Response:
```json
{
  "result": {
    "result": "\"23.11\"",
    "success": true,
    "logs": ""
  }
}
```

**POST /flake_build**

Build flake outputs (dry-run by default).

Request:
```json
{
  "flake_path": "github:nixos/nixpkgs",
  "outputs": ["packages.x86_64-linux.hello"],
  "dry_run": true  // optional, default: true
}
```

Response:
```json
{
  "result": {
    "success": true,
    "logs": "...",
    "errors": [],
    "built_paths": []  // empty if dry_run is true
  }
}
```

**POST /flake_scaffold**

Scaffold new flake projects, generate flake.nix files, or add outputs to existing flakes.

Request (Init - create new project):
```json
{
  "scaffold_type": "init",
  "template": "package",
  "target_path": "./my-project",
  "name": "my-package",
  "description": "My awesome package",
  "overwrite": false  // optional, default: false
}
```

Request (Generate - create flake.nix file):
```json
{
  "scaffold_type": "generate",
  "template": "devshell",
  "target_path": "./flake.nix",
  "name": "dev-env",
  "description": "Development environment",
  "overwrite": true
}
```

Request (AddOutput - add output to existing flake):
```json
{
  "scaffold_type": "addoutput",
  "template": "devshell",
  "target_path": "./flake.nix",
  "name": "dev-env",
  "version": "1.0.0"
}
```

Request (AddInput - add inputs to existing flake):
```json
{
  "scaffold_type": "addinput",
  "template": "package",
  "target_path": "./flake.nix",
  "inputs": [
    {
      "name": "flake-utils",
      "url": "github:numtide/flake-utils"
    }
  ]
}
```

Response:
```json
{
  "result": {
    "success": true,
    "files_created": ["./my-project/flake.nix", "./my-project/flake.lock", "./my-project/.gitignore", "./my-project/src", "./my-project/README.md"],
    "flake_content": "{ description = \"...\"; ... }",
    "logs": "Created project structure\nCreated flake.nix at ./my-project/flake.nix\nGenerated flake.lock\nFlake validation passed\n",
    "errors": [],
    "validation_passed": true
  }
}
```

### Scaffold Types

- **init**: Create a new flake project with directory structure (flake.nix, flake.lock, .gitignore, src/, README.md, default.nix)
- **generate**: Generate a flake.nix file at the specified path (optionally generates flake.lock)
- **addoutput**: Add a new output section to an existing flake.nix
- **addinput**: Add new input(s) to an existing flake.nix

### Template Types

- **package**: Basic package flake with packages output
- **devshell**: Development shell flake with devShells output
- **nixos**: NixOS configuration module flake with nixosModules output
- **multi**: Multi-output flake combining packages, apps, devShells, and lib

## Architecture

```
src/
├── main.rs              # Server entry point
├── server.rs            # MCP protocol and routing
├── models/              # Data models
│   ├── flake_input.rs
│   ├── flake_output.rs
│   ├── eval_result.rs
│   ├── build_result.rs
│   └── scaffold_result.rs
├── endpoints/           # Endpoint handlers
│   ├── flake_inputs.rs
│   ├── flake_outputs.rs
│   ├── flake_eval.rs
│   ├── flake_build.rs
│   └── flake_scaffold.rs
├── templates/           # Flake templates
│   ├── package.rs
│   ├── devshell.rs
│   ├── nixos.rs
│   └── multi.rs
└── utils/               # Utilities
    ├── nix.rs          # Nix CLI wrapper
    ├── logger.rs       # Logging utilities
    └── template.rs     # Template rendering utilities
```

## Guidelines

- All flake queries and builds follow canonical structures from nix.dev and nixos-and-flakes-book
- Dry-run builds are performed by default before applying outputs
- Structured JSON outputs are returned for agent reasoning
- Input/output names are validated against official flake conventions
- Logs provide iterative feedback and correction

## Documentation References

- [Nix Flakes Documentation](https://nixos.org/manual/nix/stable/command-ref/new-cli/nix3-flake.html)
- [nixos-and-flakes-book](https://github.com/ryan4yin/nixos-and-flakes-book)

## License

This project follows standard Rust/MIT licensing conventions.

