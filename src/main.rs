mod cli;
mod config;
mod errors;
mod mcp_client;
mod output;
mod types;

use clap::Parser;
use errors::AppError;
use serde_json::Value;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(RunOutput::Json(result)) => {
            output::print_ok(result);
            ExitCode::SUCCESS
        }
        Ok(RunOutput::Text(result)) => {
            println!("{result}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            output::print_error(&err);
            ExitCode::from(1)
        }
    }
}

enum RunOutput {
    Json(Value),
    Text(String),
}

async fn run() -> Result<RunOutput, AppError> {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Command::ListTools(args) => {
            let connection = config::resolve_connection(&args.connection, cli.config)?;
            let client = mcp_client::McpClient::new(connection);
            if args.require_capability {
                client
                    .ensure_capability(mcp_client::CapabilityKind::Tools)
                    .await?;
            }
            client.list_tools(args.cursor).await.map(RunOutput::Json)
        }
        cli::Command::ListResources(args) => {
            let connection = config::resolve_connection(&args.connection, cli.config)?;
            let client = mcp_client::McpClient::new(connection);
            if args.require_capability {
                client
                    .ensure_capability(mcp_client::CapabilityKind::Resources)
                    .await?;
            }
            client
                .list_resources(args.cursor)
                .await
                .map(RunOutput::Json)
        }
        cli::Command::ListPrompts(args) => {
            let connection = config::resolve_connection(&args.connection, cli.config)?;
            let client = mcp_client::McpClient::new(connection);
            if args.require_capability {
                client
                    .ensure_capability(mcp_client::CapabilityKind::Prompts)
                    .await?;
            }
            client.list_prompts(args.cursor).await.map(RunOutput::Json)
        }
        cli::Command::CallTool(mut args) => {
            let tool = apply_server_from_target(&mut args.connection, &args.tool);
            let connection = config::resolve_connection(&args.connection, cli.config)?;
            let params = parse_json_arg(args.params)?;
            let client = mcp_client::McpClient::new(connection);
            if args.require_capability {
                client
                    .ensure_capability(mcp_client::CapabilityKind::Tools)
                    .await?;
            }
            client.call_tool(tool, params).await.map(RunOutput::Text)
        }
        cli::Command::ReadResource(mut args) => {
            let uri = apply_server_from_target(&mut args.connection, &args.uri);
            let connection = config::resolve_connection(&args.connection, cli.config)?;
            let client = mcp_client::McpClient::new(connection);
            if args.require_capability {
                client
                    .ensure_capability(mcp_client::CapabilityKind::Resources)
                    .await?;
            }
            client.read_resource(uri).await.map(RunOutput::Text)
        }
        cli::Command::GetPrompt(mut args) => {
            let prompt_id = apply_server_from_target(&mut args.connection, &args.prompt_id);
            let connection = config::resolve_connection(&args.connection, cli.config)?;
            let params = parse_json_arg(args.params)?;
            let client = mcp_client::McpClient::new(connection);
            if args.require_capability {
                client
                    .ensure_capability(mcp_client::CapabilityKind::Prompts)
                    .await?;
            }
            client
                .get_prompt(prompt_id, params)
                .await
                .map(RunOutput::Text)
        }
        cli::Command::GetInfo(args) => {
            let connection = config::resolve_connection(&args.connection, cli.config)?;
            let client = mcp_client::McpClient::new(connection);
            client.get_info().await.map(RunOutput::Json)
        }
    }
}

fn apply_server_from_target(connection: &mut cli::ConnectionArgs, target: &str) -> String {
    if connection.server.is_none()
        && connection.url.is_none()
        && let Some((server, rest)) = split_server_target(target)
    {
        connection.server = Some(server);
        return rest;
    }

    target.to_string()
}

fn split_server_target(target: &str) -> Option<(String, String)> {
    if target.contains("://") {
        return None;
    }

    let (server, rest) = target.split_once(':')?;

    if server.is_empty() || rest.is_empty() {
        return None;
    }

    Some((server.to_string(), rest.to_string()))
}

fn parse_json_arg(raw: Option<String>) -> Result<Option<Value>, AppError> {
    let Some(raw) = raw else {
        return Ok(None);
    };

    let value = serde_json::from_str(&raw)
        .map_err(|err| AppError::new("invalid_json", format!("Invalid JSON: {err}")))?;
    Ok(Some(value))
}
