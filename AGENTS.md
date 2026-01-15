# AGENTS

## Project

- `call-mcp` is a Rust CLI that bridges Agent Skills-style workflows to MCP servers.
- Only Streamable HTTP transport is supported (no stdio).
- Success output is YAML for the result; errors are YAML with `code`, `message`, optional `details`.
- `call-tool`, `get-prompt`, and `read-resource` return plain text on success; `call-tool` streams progress/log notifications.

## Commands

- `list-tools`, `list-resources`, `list-prompts`
- `call-tool <tool>`, `read-resource <uri>`, `get-prompt <prompt-id>`
- `get-info`

## Configuration

- Default config file: `.mcp.json` (or `mcp.json`).
- Server entries use `{ "type": "http", "url": "http://host/mcp", "headers": { ... } }`.
- CLI flags allow overrides: `--config`, `--server`, `--url`, `--header`, `--token-env`, timeouts, retries.

## Layout

- `src/main.rs`: entry point and command dispatch.
- `src/cli.rs`: CLI definitions and args.
- `src/config.rs`: config loading and server resolution.
- `src/output.rs`: YAML output formatting.
- `src/mcp_client/`: Streamable HTTP client, per-command modules.

## Workflow

- After any code change: run `cargo fmt`, then `cargo clippy`, then `cargo build` (debug only).
- Do not build release artifacts; release builds are manual by the user.
