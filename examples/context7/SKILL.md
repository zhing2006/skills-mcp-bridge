---
name: context7
description: Query up-to-date documentation and code examples for any programming library or framework using Context7 MCP service. Use this skill when you need current documentation, API references, or code examples for libraries like React, Next.js, MongoDB, etc.
---

# Context7 Documentation Query

This skill enables querying the Context7 documentation service to retrieve up-to-date documentation and code examples for programming libraries.

## Important

- Use forward slashes (`/`) for all script's paths, even on Windows.

## Available Tools

### resolve-library-id

Resolves a package/product name to a Context7-compatible library ID and returns matching libraries.

You MUST call this function before 'query-docs' to obtain a valid Context7-compatible library ID UNLESS the user explicitly provides a library ID in the format '/org/project' or '/org/project/version' in their query.

### query-docs

Retrieves and queries up-to-date documentation and code examples from Context7 for any programming library or framework.

You must call 'resolve-library-id' first to obtain the exact Context7-compatible library ID required to use this tool, UNLESS the user explicitly provides a library ID in the format '/org/project' or '/org/project/version' in their query.

## Workflow

### STEP 1: Get Detail Information and Call Schema

```bash
{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json list-tools --server context7 --name {tool_name}
```

### STEP 2: Call the Tool

```bash
{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json call-tool context7:{tool_name} --params '{...}'
```

## Guidelines

- Always resolve the library ID first before querying docs
- Be specific in your queries for better results
- Always check tool parameters with `list-tools --name {tool_name}` before calling
