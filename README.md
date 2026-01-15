# Agent Skills to MCP Bridge

[English](README.md) | [中文](README_CN.md)

call-mcp is a Rust CLI that bridges Agent Skills-style workflows to MCP servers. The goal is to wrap traditional MCP services in a way that is easy for AI-driven skills to invoke at runtime.

## Design Purpose

- Provide a stable CLI for Agent Skills to call MCP tools, read resources, and fetch prompts.
- Keep output human/AI-friendly (YAML) while preserving structured content.
- Prefer remote MCP over HTTP so skills can reach external services reliably.

## Why Streamable HTTP Only

- Stdio MCP servers are local by design. A one-shot skill invocation would spawn a new server each time, which is wasteful and brittle.
- If a capability is local, it is usually simpler to implement the behavior directly in the skill instead of bridging a stdio server.
- Streamable HTTP is the right fit for long-lived remote MCP services.

## Build

Install Rust (via rustup) if you cloned the repo:

```bash
curl https://sh.rustup.rs -sSf | sh
```

On Windows (PowerShell, requires winget):

```powershell
winget install Rustlang.Rustup
```

Build the CLI:

```bash
# Build debug version
cargo build

# Build release version
cargo build -r
```

The binary is written to:

- Windows: `target/debug/call-mcp.exe`, `target/release/call-mcp.exe`
- macOS/Linux: `target/debug/call-mcp`, `target/release/call-mcp`

## Configuration

Default config file: `.mcp.json` (or `mcp.json`). You can also pass `--config` explicitly.

Example for Context7 (uses headers):

```json
{
  "mcpServers": {
    "context7": {
      "type": "http",
      "url": "https://mcp.context7.com/mcp",
      "headers": {
        "CONTEXT7_API_KEY": "ctx7sk-xxx"
      }
    }
  }
}
```

## Commands

- `list-tools`
- `call-tool <tool>`
- `list-resources`
- `read-resource <uri>`
- `list-prompts`
- `get-prompt <prompt-id>`
- `get-info`

Common flags:

- `--server <name>`
- `--url <url>`
- `--config <path>`
- `--header "Name: Value"` (repeatable)
- `--token-env <ENV_VAR>` (adds `Authorization: Bearer <token>`)
- `--timeout <ms>` / `--connect-timeout <ms>`
- `--retry <count>` / `--retry-backoff <ms>`
- `--name <name>` (filter list-tools/resources/prompts by name)
- `--require-capability` (checks server capabilities before calling)

You can also use `<server>:<tool>` or `<server>:<prompt>` to avoid `--server`.

## Output Format

- Success: YAML of the result only (no `ok/result` wrapper).
- Error: YAML with `code`, `message`, and optional `details`.

Example error:

```yaml
code: mcp_service
message: "Mcp error: -32601: Method not found"
```

## Skill Development

Use the examples in `examples/` as templates. Each skill should include:

- `SKILL.md` with frontmatter (`name`, `description`) and a short workflow.
- `assets/mcp.json` for per-skill MCP configuration.

Typical pattern:

```bash
{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json list-tools --server <server> --name <tool>
{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json call-tool <server>:<tool> --params '{...}'
```

Notes:

- Use forward slashes in skill scripts, even on Windows.
- Prefer `--name` when you want a single tool/prompt/resource.

## Examples (Context7)

List tools:

```powershell
./target/debug/call-mcp.exe --config .mcp.json list-tools --server context7 --require-capability
```

Resolve a library ID:

```powershell
./target/debug/call-mcp.exe --config .mcp.json call-tool context7:resolve-library-id --require-capability --params '{"libraryName":"react","query":"How to use useEffect cleanup?"}'
```

Query docs (use the libraryId from the previous step):

```powershell
./target/debug/call-mcp.exe --config .mcp.json call-tool context7:query-docs --require-capability --params '{"libraryId":"/facebook/react","query":"useEffect cleanup examples"}'
```

Get server info (capabilities, etc.):

```powershell
./target/debug/call-mcp.exe --config .mcp.json get-info --server context7
```

Notes:

- Use `--require-capability` to avoid calling unsupported commands.
