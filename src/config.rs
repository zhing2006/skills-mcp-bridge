use crate::cli::ConnectionArgs;
use crate::errors::AppError;
use crate::types::Header;
use crate::user_agent::UserAgentPreset;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    #[serde(rename = "mcpServers")]
    pub mcp_servers: HashMap<String, ServerEntry>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct ServerEntry {
    #[serde(rename = "type")]
    pub server_type: Option<String>,
    pub url: Option<String>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default)]
    pub token_env: Option<String>,
    #[serde(default, alias = "timeout_ms")]
    pub timeout: Option<u64>,
    #[serde(default, alias = "connect_timeout_ms")]
    pub connect_timeout: Option<u64>,
    #[serde(default)]
    pub retry: Option<u32>,
    #[serde(default, alias = "retry_backoff_ms")]
    pub retry_backoff: Option<u64>,
    #[serde(default)]
    pub user_agent: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ResolvedConnection {
    pub url: String,
    pub headers: Vec<Header>,
    pub timeout: Option<u64>,
    pub connect_timeout: Option<u64>,
    pub retry: Option<u32>,
    pub retry_backoff: Option<u64>,
    pub user_agent: UserAgentPreset,
}

pub fn resolve_connection(
    args: &ConnectionArgs,
    config_path: Option<PathBuf>,
) -> Result<ResolvedConnection, AppError> {
    let config = load_config(config_path)?;
    let mut url = args.url.clone();
    let mut headers: Vec<Header> = Vec::new();
    let mut token_env = args.token_env.clone();
    let mut timeout = args.timeout;
    let mut connect_timeout = args.connect_timeout;
    let mut retry = args.retry;
    let mut retry_backoff = args.retry_backoff;
    let mut user_agent = args.user_agent.clone();

    if let Some(server) = &args.server {
        let Some(config) = config.as_ref() else {
            return Err(AppError::new(
                "config_missing",
                "Server specified but no config file found",
            ));
        };

        let entry = config.mcp_servers.get(server).ok_or_else(|| {
            AppError::new("server_not_found", format!("Server not found: {server}"))
        })?;

        if url.is_none() {
            url = entry.url.clone();
        }

        for (name, value) in &entry.headers {
            headers.push(Header::new(name, value));
        }

        if token_env.is_none() {
            token_env = entry.token_env.clone();
        }
        if timeout.is_none() {
            timeout = entry.timeout;
        }
        if connect_timeout.is_none() {
            connect_timeout = entry.connect_timeout;
        }
        if retry.is_none() {
            retry = entry.retry;
        }
        if retry_backoff.is_none() {
            retry_backoff = entry.retry_backoff;
        }
        if user_agent.is_none() {
            if let Some(ua_str) = &entry.user_agent {
                user_agent = Some(ua_str.parse().map_err(|e| {
                    AppError::new(
                        "invalid_user_agent",
                        format!("Invalid user_agent value: {e}"),
                    )
                })?);
            }
        }
    }

    headers.extend(args.headers.iter().cloned());

    if let Some(token_env) = token_env {
        let token = std::env::var(&token_env).map_err(|_| {
            AppError::new("token_missing", format!("Token env not set: {token_env}"))
        })?;

        if !header_exists(&headers, "Authorization") {
            headers.push(Header::new("Authorization", format!("Bearer {token}")));
        }
    }

    let url =
        url.ok_or_else(|| AppError::new("missing_connection", "Missing --url or --server value"))?;

    // Default to Chrome if not specified
    let user_agent = user_agent.unwrap_or_default();

    Ok(ResolvedConnection {
        url,
        headers,
        timeout,
        connect_timeout,
        retry,
        retry_backoff,
        user_agent,
    })
}

fn load_config(path: Option<PathBuf>) -> Result<Option<ConfigFile>, AppError> {
    let mut candidates = Vec::new();

    if let Some(path) = path {
        if !path.exists() {
            return Err(AppError::new(
                "config_not_found",
                format!("Config file not found: {}", path.display()),
            ));
        }
        candidates.push(path);
    } else {
        candidates.push(PathBuf::from(".mcp.json"));
        candidates.push(PathBuf::from("mcp.json"));
    }

    for path in candidates {
        if !path.exists() {
            continue;
        }

        let contents = std::fs::read_to_string(&path).map_err(|err| {
            AppError::new(
                "config_read",
                format!("Failed to read config: {} ({err})", path.display()),
            )
        })?;

        let config = serde_json::from_str(&contents).map_err(|err| {
            AppError::new(
                "config_parse",
                format!("Failed to parse config: {} ({err})", path.display()),
            )
        })?;

        return Ok(Some(config));
    }

    Ok(None)
}

fn header_exists(headers: &[Header], name: &str) -> bool {
    headers
        .iter()
        .any(|header| header.name.eq_ignore_ascii_case(name))
}
