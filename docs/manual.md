# Skills MCP Bridge - Wrap MCP Services with Agent Skills

## Why This Project?

### 1. Smooth Transition for Production Environments

Agent Skills is a newly released next-generation AI capability extension. However, the reality is: **many production environments already have mature MCP services deployed**. These services are battle-tested and running stably—rewriting them all to Skills format in the short term is simply not feasible.

call-mcp provides a bridging solution: **call existing MCP services using the Skills approach**, without rewriting any MCP server code, enabling a smooth transition.

### 2. Save Context Space

MCP protocol has a notable issue: **tool definitions consume significant context space**. The complete JSON Schema for each MCP tool must be passed to the AI model. When a service provides multiple tools, these definitions can consume thousands of tokens.

Agent Skills adopts a **Progressive Disclosure** design:

- Initially only loads brief descriptions from SKILL.md
- AI fetches detailed parameters for individual tools via `list-tools --name <tool>` only when needed
- Avoids loading all tool definitions at once

This design **significantly reduces context usage**, giving AI more space to handle actual tasks.

## Why HTTP Streamable Only?

This project **only supports remote HTTP MCP services**, not local stdio MCP. The reasons are simple:

- **stdio MCP runs as local processes**: Each invocation requires spawning a new server process—inefficient and unstable
- **Local capabilities don't need bridging**: If a capability is local, it can be implemented directly as a Skill—no need to go through MCP
- **HTTP suits remote services**: Long-running remote MCP services are the target use case

In short: **implement locally with Skills directly**, **bridge remote services with call-mcp**.

## Installation

### Prerequisites

Install the Rust toolchain:

**Linux/macOS:**

```bash
curl https://sh.rustup.rs -sSf | sh
```

**Windows (PowerShell):**

```powershell
winget install Rustlang.Rustup
```

### Build

```bash
# Clone the repository
git clone https://github.com/anthropics/skills-mcp-bridge.git
cd skills-mcp-bridge

# Build release version
cargo build -r
```

Binary locations:

- Windows: `target/release/call-mcp.exe`
- macOS/Linux: `target/release/call-mcp`

### Deploy to Skill Directory

Copy the compiled `call-mcp` executable to your Skill directory, alongside `SKILL.md` and `assets/mcp.json`.

## How to Create Your Own MCP Skill

Refer to examples in the `examples/` directory. Creating your own MCP Skill takes just 3 steps:

### Step 1: Create Directory Structure

```txt
my-skill/
├── call-mcp|call-mcp.exe # Compiled executable
├── SKILL.md              # Skill description file
└── assets/
    └── mcp.json          # MCP server configuration
```

### Step 2: Configure MCP Server

Create `assets/mcp.json` to configure your MCP service:

```json
{
  "mcpServers": {
    "your-service": {
      "type": "http",
      "url": "https://your-mcp-server.com/mcp",
      "headers": {
        "Authorization": "Bearer your-api-key"
      }
    }
  }
}
```

Supported authentication methods:

- `headers`: Custom HTTP headers (including API keys)
- `--token-env`: Read Bearer Token from environment variable

User-Agent setting (default: Chrome):

- `--user-agent chrome`: Chrome browser (default)
- `--user-agent claude-code`: Claude Code CLI
- `--user-agent codex`: OpenAI Codex CLI
- `--user-agent gemini-cli`: Google Gemini CLI
- `--user-agent opencode`: OpenCode AI
- `--user-agent cursor`: Cursor editor
- More options: `edge`, `firefox`, `safari`, `ie`
- Custom User-Agent: `--user-agent "MyApp/1.0"`

### Step 3: Write SKILL.md

SKILL.md is the core of a Skill, containing frontmatter metadata and usage instructions:

```markdown
---
name: your-skill-name
description: Brief description of what this Skill does. AI uses this to decide when to invoke it.
---

# Your Skill Name

Describe the capabilities this Skill provides.

## Workflow

### STEP 1: Get All Available Tools

If you already have the tools list, you can skip this step and proceed to STEP 2

{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json list-tools --server your-service --short

### STEP 2: Get Detail Information and Call Schema

If you already have the call schema, you can skip this step and proceed to STEP 3

{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json list-tools --server your-service --name {tool_name}

### STEP 3: Call the Tool

{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json call-tool your-service:{tool_name} --params '{...}'

## Guidelines

- Usage tips and notes
```

## Example Walkthrough: Context7 Documentation Query

Using `examples/context7/` as an example, here's a complete guide on wrapping the Context7 MCP service as a Skill.

### Directory Structure

```txt
context7/
├── call-mcp|call-mcp.exe # Compiled executable
├── SKILL.md              # Skill description file
└── assets/
    └── mcp.json          # MCP server configuration
```

### mcp.json Configuration

```json
{
  "mcpServers": {
    "context7": {
      "type": "http",
      "url": "https://mcp.context7.com/mcp",
      "headers": {
        "CONTEXT7_API_KEY": "ctx7sk-your-api-key-here"
      }
    }
  }
}
```

**Configuration fields:**

- `type`: Must be `http`, this project only supports HTTP Streamable
- `url`: MCP service endpoint, obtained from the service provider's documentation
- `headers`: HTTP request headers for authentication
- `user_agent`: Optional User-Agent setting (e.g., `chrome`, `claude-code`, `codex`, defaults to `chrome`)

**How to get the API Key:**

1. Visit [Context7 website](https://context7.com) and create an account
2. Create an API Key in the console (format: `ctx7sk-xxx`)
3. Enter the key in the `CONTEXT7_API_KEY` field

> Different MCP services use different authentication methods. Some use custom headers (like Context7), others use standard `Authorization: Bearer <token>`. Refer to the respective service documentation.

### SKILL.md Breakdown

#### Frontmatter Metadata

```yaml
---
name: context7
description: Query up-to-date documentation and code examples for any programming library or framework using Context7 MCP service.
---
```

- `name`: Unique identifier for the Skill
- `description`: AI uses this to decide when to invoke the Skill (critical! be clear about the purpose)

#### Workflow Section

```bash
# STEP 1: Get all available tools (short mode for discovery)
{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json list-tools --server context7 --short

# STEP 2: Get detailed parameter Schema for the tool
{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json list-tools --server context7 --name {tool_name}

# STEP 3: Call the tool
{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json call-tool context7:{tool_name} --params '{...}'
```

- `{skill_dir}`: Automatically replaced by the framework with the actual Skill path
- `{tool_name}`: AI replaces with the specific tool name as needed
- `--short`: Returns only name and description for each tool, enabling quick discovery without loading full schemas

#### Guidelines Section

```markdown
- Always resolve the library ID first before querying docs
- Always check tool parameters with `list-tools --name {tool_name}` before calling
```

Provides usage tips to help AI use tools correctly.

### Actual Invocation Flow

Suppose a user asks: "How do I use React's useEffect?"

**1. AI first discovers available tools:**

```bash
./call-mcp --config assets/mcp.json list-tools --server context7 --short
```

Returns a list of tools with names and descriptions. AI sees `resolve-library-id` and `query-docs`.

**2. AI gets resolve-library-id parameters:**

```bash
./call-mcp --config assets/mcp.json list-tools --server context7 --name resolve-library-id
```

Returns the complete JSON Schema for the tool. AI learns it needs the `libraryName` parameter.

**3. AI calls resolve-library-id:**

```bash
./call-mcp --config assets/mcp.json call-tool context7:resolve-library-id \
  --params '{"libraryName":"react"}'
```

Returns: `/facebook/react`

**4. AI gets query-docs parameters:**

```bash
./call-mcp --config assets/mcp.json list-tools --server context7 --name query-docs
```

**5. AI calls query-docs:**

```bash
./call-mcp --config assets/mcp.json call-tool context7:query-docs \
  --params '{"libraryId":"/facebook/react","query":"useEffect usage"}'
```

Returns the latest React useEffect documentation and example code.

### Key Design Points

1. **Be precise with description**: AI decides when to use based on this text—unclear descriptions won't get invoked
2. **Use `--short` for discovery**: First get all tools with `--short` to see names and descriptions without loading full schemas
3. **Query before calling**: Use `list-tools --name` to get detailed parameters for specific tools
4. **Use forward slashes**: Even on Windows, use `/` for cross-platform compatibility

## Summary

call-mcp enables you to:

- Reuse existing MCP services without rewriting
- Save context space through Skills' progressive disclosure
- Get production-grade reliability (retry, timeout, capability checks)
- Quickly wrap any HTTP MCP service as an Agent Skill
