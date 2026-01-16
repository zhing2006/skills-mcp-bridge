---
name: firecrawl
description: Web scraping and content extraction using Firecrawl service. Use this skill when you need to scrape web pages, crawl websites, search the web, or extract structured data from URLs.
---

# Firecrawl Web Scraping

This skill enables web scraping, crawling, and content extraction using the Firecrawl service.

## Important

- Use forward slashes (`/`) for all script's paths, even on Windows.

## Workflow

### STEP 1: Get All Available Tools

If you already have the tools list, you can skip this step and proceed to STEP 2

```bash
{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json list-tools --server firecrawl --short
```

### STEP 2: Get Detail Information and Call Schema

If you already have the call schema, you can skip this step and proceed to STEP 3

```bash
{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json list-tools --server firecrawl --name {tool_name}
```

### STEP 3: Call the Tool

```bash
{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json call-tool firecrawl:{tool_name} --params '{...}'
```

## Guidelines

- Use `firecrawl_scrape` for single known URLs - it's the fastest and most reliable
- Use `firecrawl_search` when you don't know which site has the information
- Use `firecrawl_map` to discover site structure before crawling
- Be mindful of crawl limits to avoid large responses that exceed token limits
- Use `firecrawl_extract` when you need structured data output with a JSON schema
- Use `firecrawl_agent` for complex multi-step data gathering tasks
- Always check tool parameters with `list-tools --name {tool_name}` before calling
