---
name: context7
description: Query up-to-date documentation and code examples for any programming library or framework using Context7 MCP service. Use this skill when you need current documentation, API references, or code examples for libraries like React, Next.js, MongoDB, etc.
---

# Context7 Documentation Query

This skill enables querying the Context7 documentation service to retrieve up-to-date documentation and code examples for programming libraries.

## Important

- Use forward slashes (`/`) for all script's paths, even on Windows.

## Workflow

### STEP 1: Get All Available Tools

If you already have the tools list, you can skip this step and proceed to STEP 2

```bash
{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json list-tools --server context7 --short
```

### STEP 2: Get Detail Information and Call Schema

If you already have the call schema, you can skip this step and proceed to STEP 3

```bash
{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json list-tools --server context7 --name {tool_name}
```

### STEP 3: Call the Tool

```bash
{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json call-tool context7:{tool_name} --params '{...}'
```

## Guidelines

- Always resolve the library ID first before querying docs
- Be specific in your queries for better results
- Always check tool parameters with `list-tools --name {tool_name}` before calling
