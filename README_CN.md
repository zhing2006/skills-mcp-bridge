# Agent Skills to MCP Bridge

[English](README.md) | [中文](README_CN.md)

call-mcp 是一个 Rust CLI，用于把传统 MCP 服务封装成可被 Agent Skills 调用的形式，方便 AI 在运行时使用 MCP 能力。

## 设计目的

- 为 Agent Skills 提供稳定的命令行接口，调用 MCP 的 tools/resources/prompts。
- 输出更适合 AI 阅读（YAML），同时保留结构化信息。
- 优先面向远程 MCP 服务，避免本地桥接的复杂度。

## 为什么只支持 Streamable HTTP

- Stdio MCP 是本地进程模型，一次性 skill 调用会导致每次都要启动 server，成本高且不稳定。
- 本地能力通常可以直接在 skill 里实现，不必桥接 stdio server。
- Streamable HTTP 更适合远程 MCP 服务与长期连接。

## 构建

如果是从源码拉取，先安装 Rust（建议用 rustup）：

```bash
curl https://sh.rustup.rs -sSf | sh
```

Windows（PowerShell，需要 winget）：

```powershell
winget install Rustlang.Rustup
```

编译：

```bash
# 构建 Debug 版
cargo build

# 构建 Release 版
cargo build -r
```

可执行文件位置：

- Windows: `target//debug//call-mcp.exe`, `target//release//call-mcp.exe`
- macOS/Linux: `target/debug/call-mcp`， `target/release/call-mcp`

## 配置

默认读取 `.mcp.json`（或 `mcp.json`），也可以用 `--config` 指定。

Context7 示例（需要 headers）：

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

## 命令

- `list-tools`
- `call-tool <tool>`
- `list-resources`
- `read-resource <uri>`
- `list-prompts`
- `get-prompt <prompt-id>`
- `get-info`

常用参数：

- `--server <name>`
- `--url <url>`
- `--config <path>`
- `--header "Name: Value"`（可重复）
- `--token-env <ENV_VAR>`（自动加 `Authorization: Bearer <token>`）
- `--timeout <ms>` / `--connect-timeout <ms>`
- `--retry <count>` / `--retry-backoff <ms>`
- `--name <name>`（仅用于 list 命令，按名称过滤）
- `--require-capability`（调用前检查服务端能力）

也可以使用 `<server>:<tool>` 或 `<server>:<prompt>` 来省略 `--server`。

## 输出格式

- 成功：直接输出结果的 YAML（不再包 `ok/result`）。
- 失败：输出 `code` / `message` / `details`（如果有）。

错误示例：

```yaml
code: mcp_service
message: "Mcp error: -32601: Method not found"
```

## Skill 开发

参考 `examples/` 里的模板。一个 skill 通常包含：

- `SKILL.md`（带 frontmatter：`name`、`description`，再写简短流程）
- `assets/mcp.json`（每个 skill 的 MCP 配置）

常用调用模式：

```bash
{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json list-tools --server <server> --name <tool>
{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json call-tool <server>:<tool> --params '{...}'
```

说明：

- skill 脚本路径统一用 `/`，即使在 Windows。
- 只想拿单个 tool/prompt/resource 时优先用 `--name`。

## 使用示例（Context7）

列出工具：

```powershell
./target/debug/call-mcp.exe --config .mcp.json list-tools --server context7 --require-capability
```

解析库 ID：

```powershell
./target/debug/call-mcp.exe --config .mcp.json call-tool context7:resolve-library-id --require-capability --params '{"libraryName":"react","query":"How to use useEffect cleanup?"}'
```

查询文档（使用上一步返回的 libraryId）：

```powershell
./target/debug/call-mcp.exe --config .mcp.json call-tool context7:query-docs --require-capability --params '{"libraryId":"/facebook/react","query":"useEffect cleanup examples"}'
```

查看服务端信息（capabilities 等）：

```powershell
./target/debug/call-mcp.exe --config .mcp.json get-info --server context7
```

说明：

- 建议使用 `--require-capability` 避免调用不支持的命令。
