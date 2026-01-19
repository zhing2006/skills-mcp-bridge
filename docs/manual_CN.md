# Skills MCP Bridge - 用 Agent Skills 包装 MCP 服务

## 为什么做这个项目？

### 1. 生产环境的平滑过渡

Agent Skills 作为新一代的 AI 能力扩展方案刚刚推出，但现实是：**大量生产环境中已经部署了成熟的 MCP 服务**。这些服务经过验证、稳定运行，短期内不可能全部重写为 Skills 格式。

call-mcp 提供了一个桥接方案：**让你可以用 Skills 的方式调用现有的 MCP 服务**，无需重写任何 MCP 服务端代码，实现平滑过渡。

### 2. 节省上下文空间

MCP 协议有一个显著问题：**工具定义占用大量上下文空间**。每个 MCP 工具的完整 JSON Schema 都需要传递给 AI 模型，当服务提供多个工具时，这些定义可能消耗数千 tokens。

Agent Skills 采用**渐进式披露（Progressive Disclosure）**设计：

- 初始时只加载 SKILL.md 中的简短描述
- AI 需要时才通过 `list-tools --name <tool>` 获取单个工具的详细参数
- 避免一次性加载所有工具定义

这种设计可以**显著减少上下文占用**，让 AI 有更多空间处理实际任务。

## 为什么只支持 HTTP Streamable？

本项目**仅支持远程 HTTP MCP 服务**，不支持本地 stdio MCP。原因很简单：

- **stdio MCP 是本地进程**：每次调用都需要启动一个新的服务器进程，效率低且不稳定
- **本地能力无需桥接**：如果某个能力是本地的，完全可以通过 Skills 方式直接实现，没必要绕道 MCP
- **HTTP 适合远程服务**：长期运行的远程 MCP 服务才是本项目的目标场景

简言之：**本地用 Skills 直接实现**，**远程用 call-mcp 桥接 MCP**。

## 安装方法

### 前置条件

安装 Rust 工具链：

**Linux/macOS:**

```bash
curl https://sh.rustup.rs -sSf | sh
```

**Windows (PowerShell):**

```powershell
winget install Rustlang.Rustup
```

### 编译

```bash
# 克隆仓库
git clone https://github.com/your-repo/skills-mcp-bridge.git
cd skills-mcp-bridge

# 编译 release 版本
cargo build -r
```

编译产物位置：

- Windows: `target/release/call-mcp.exe`
- macOS/Linux: `target/release/call-mcp`

### 部署到 Skill 目录

将编译好的 `call-mcp` 可执行文件复制到你的 Skill 目录中，与 `SKILL.md` 和 `assets/mcp.json` 放在一起。

## 如何创建自己的 MCP Skill

参考 `examples/` 目录中的示例，创建自己的 MCP Skill 只需 3 步：

### 步骤 1：创建目录结构

```txt
my-skill/
├── call-mcp|call-mcp.exe # 编译好的可执行文件
├── SKILL.md              # Skill 描述文件
└── assets/
    └── mcp.json          # MCP 服务器配置
```

### 步骤 2：配置 MCP 服务器

创建 `assets/mcp.json`，配置你的 MCP 服务：

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

支持的认证方式：

- `headers`: 自定义 HTTP 头（包括 API Key）
- `--token-env`: 从环境变量读取 Bearer Token

User-Agent 设置（默认: Chrome）：

- `--user-agent chrome`: Chrome 浏览器（默认）
- `--user-agent claude-code`: Claude Code CLI
- `--user-agent codex`: OpenAI Codex CLI
- `--user-agent gemini-cli`: Google Gemini CLI
- `--user-agent opencode`: OpenCode AI
- `--user-agent cursor`: Cursor 编辑器
- 更多选项：`edge`, `firefox`, `safari`, `ie`
- 自定义 User-Agent：`--user-agent "MyApp/1.0"`

### 步骤 3：编写 SKILL.md

SKILL.md 是 Skill 的核心，包含 frontmatter 元数据和使用说明：

```markdown
---
name: your-skill-name
description: 简短描述这个 Skill 的用途，AI 会根据这个描述决定何时使用
---

# Your Skill Name

描述这个 Skill 提供的能力。

## Workflow

### STEP 1: 获取所有可用工具

如果你已经知道有哪些工具，可以跳过此步骤直接进入 STEP 2

{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json list-tools --server your-service --short

### STEP 2: 获取工具详情和调用参数

如果你已经知道调用参数，可以跳过此步骤直接进入 STEP 3

{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json list-tools --server your-service --name {tool_name}

### STEP 3: 调用工具

{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json call-tool your-service:{tool_name} --params '{...}'

## Guidelines

- 使用建议和注意事项
```

## 示例详解：Context7 文档查询

以 `examples/context7/` 为例，完整讲解如何将 Context7 MCP 服务包装为 Skill。

### 目录结构

```txt
context7/
├── call-mcp|call-mcp.exe # 编译好的可执行文件
├── SKILL.md              # Skill 描述文件
└── assets/
    └── mcp.json          # MCP 服务器配置
```

### mcp.json 配置

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

**配置说明：**

- `type`: 必须为 `http`，本项目只支持 HTTP Streamable
- `url`: MCP 服务的端点地址，从服务提供商文档获取
- `headers`: HTTP 请求头，用于传递认证信息
- `user_agent`: 可选，User-Agent 设置（如 `chrome`, `claude-code`, `codex` 等，默认 `chrome`）

**API Key 获取方式：**

1. 访问 [Context7 官网](https://context7.com) 注册账号
2. 在控制台创建 API Key（格式为 `ctx7sk-xxx`）
3. 将 Key 填入 `CONTEXT7_API_KEY` 字段

> 不同的 MCP 服务认证方式不同。有些用自定义 Header（如 Context7），有些用标准的 `Authorization: Bearer <token>`。请参考对应服务的文档。

### SKILL.md 解析

#### Frontmatter 元数据

```yaml
---
name: context7
description: Query up-to-date documentation and code examples for any programming library or framework using Context7 MCP service.
---
```

- `name`: Skill 的唯一标识符
- `description`: AI 根据这段描述决定何时调用此 Skill（关键！写清楚用途）

#### Workflow 章节

```bash
# STEP 1: 获取所有可用工具（简短模式，用于发现）
{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json list-tools --server context7 --short

# STEP 2: 获取工具的详细参数 Schema
{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json list-tools --server context7 --name {tool_name}

# STEP 3: 调用工具
{skill_dir}/call-mcp --config {skill_dir}/assets/mcp.json call-tool context7:{tool_name} --params '{...}'
```

- `{skill_dir}`: 框架自动替换为 Skill 的实际路径
- `{tool_name}`: AI 根据需要替换为具体工具名
- `--short`: 只返回每个工具的名称和描述，用于快速发现，无需加载完整 Schema

#### Guidelines 章节

```markdown
- Always resolve the library ID first before querying docs
- Always check tool parameters with `list-tools --name {tool_name}` before calling
```

提供使用建议，帮助 AI 正确使用工具。

### 实际调用流程

假设用户问："React 的 useEffect 怎么用？"

**1. AI 先发现可用工具：**

```bash
./call-mcp --config assets/mcp.json list-tools --server context7 --short
```

返回工具列表，包含名称和描述。AI 看到有 `resolve-library-id` 和 `query-docs`。

**2. AI 获取 resolve-library-id 的参数：**

```bash
./call-mcp --config assets/mcp.json list-tools --server context7 --name resolve-library-id
```

返回工具的完整 JSON Schema，AI 知道需要传 `libraryName` 参数。

**3. AI 调用 resolve-library-id：**

```bash
./call-mcp --config assets/mcp.json call-tool context7:resolve-library-id \
  --params '{"libraryName":"react"}'
```

返回：`/facebook/react`

**4. AI 获取 query-docs 的参数：**

```bash
./call-mcp --config assets/mcp.json list-tools --server context7 --name query-docs
```

**5. AI 调用 query-docs：**

```bash
./call-mcp --config assets/mcp.json call-tool context7:query-docs \
  --params '{"libraryId":"/facebook/react","query":"useEffect usage"}'
```

返回 React useEffect 的最新文档和示例代码。

### 设计要点

1. **description 要精准**：AI 靠这段话判断何时使用，写不清楚就不会被调用
2. **用 `--short` 发现工具**：先用 `--short` 获取所有工具的名称和描述，无需加载完整 Schema
3. **先查后调**：用 `list-tools --name` 获取特定工具的详细参数
4. **路径用正斜杠**：即使 Windows 也用 `/`，确保跨平台兼容

## 总结

call-mcp 让你能够：

- 复用现有的 MCP 服务，无需重写
- 通过 Skills 的渐进式披露节省上下文空间
- 获得生产级的可靠性（重试、超时、能力检查）
- 快速将任何 HTTP MCP 服务包装为 Agent Skill
