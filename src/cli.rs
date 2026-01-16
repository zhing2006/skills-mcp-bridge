use crate::types::Header;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "call-mcp", version, about = "MCP client bridge CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[arg(long)]
    pub config: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    ListTools(ListArgs),
    CallTool(CallToolArgs),
    ListResources(ListArgs),
    ReadResource(ReadResourceArgs),
    ListPrompts(ListArgs),
    GetPrompt(GetPromptArgs),
    GetInfo(GetInfoArgs),
}

#[derive(Debug, Args)]
pub struct ListArgs {
    #[command(flatten)]
    pub connection: ConnectionArgs,

    #[arg(long)]
    pub cursor: Option<String>,

    #[arg(long)]
    pub name: Option<String>,

    #[arg(long)]
    pub require_capability: bool,

    /// Short mode: only show name and description for tool discovery
    #[arg(long)]
    pub short: bool,
}

#[derive(Debug, Args)]
pub struct CallToolArgs {
    #[command(flatten)]
    pub connection: ConnectionArgs,

    pub tool: String,

    #[arg(long)]
    pub params: Option<String>,

    #[arg(long)]
    pub require_capability: bool,
}

#[derive(Debug, Args)]
pub struct ReadResourceArgs {
    #[command(flatten)]
    pub connection: ConnectionArgs,

    pub uri: String,

    #[arg(long)]
    pub require_capability: bool,
}

#[derive(Debug, Args)]
pub struct GetPromptArgs {
    #[command(flatten)]
    pub connection: ConnectionArgs,

    pub prompt_id: String,

    #[arg(long)]
    pub params: Option<String>,

    #[arg(long)]
    pub require_capability: bool,
}

#[derive(Debug, Args)]
pub struct GetInfoArgs {
    #[command(flatten)]
    pub connection: ConnectionArgs,
}

#[derive(Debug, Args, Clone)]
pub struct ConnectionArgs {
    #[arg(long)]
    pub server: Option<String>,

    #[arg(long)]
    pub url: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<Header>,

    #[arg(long)]
    pub token_env: Option<String>,

    #[arg(long)]
    pub timeout: Option<u64>,

    #[arg(long)]
    pub connect_timeout: Option<u64>,

    #[arg(long)]
    pub retry: Option<u32>,

    #[arg(long)]
    pub retry_backoff: Option<u64>,

    #[arg(long)]
    pub client_name: Option<String>,

    #[arg(long)]
    pub client_version: Option<String>,
}

fn parse_header(raw: &str) -> Result<Header, String> {
    let mut parts = raw.splitn(2, ':');
    let name = parts.next().unwrap_or_default().trim();
    let value = parts.next().unwrap_or_default().trim();

    if name.is_empty() {
        return Err("Header name is required (use \"Name: Value\")".to_string());
    }

    Ok(Header::new(name, value))
}
