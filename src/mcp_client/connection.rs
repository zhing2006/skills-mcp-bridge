use crate::config::ResolvedConnection;
use crate::errors::AppError;
use crate::types::Header;
use backoff::ExponentialBackoff;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use rmcp::model::ClientInfo;
use std::time::Duration;

pub(crate) fn build_backoff(base_delay_ms: Option<u64>) -> ExponentialBackoff {
    let mut backoff = ExponentialBackoff::default();
    if let Some(delay) = base_delay_ms {
        backoff.initial_interval = Duration::from_millis(delay);
    }
    backoff
}

pub(crate) fn build_http_client(
    headers: &HeaderMap,
    timeout_ms: Option<u64>,
    connect_timeout_ms: Option<u64>,
) -> Result<reqwest::Client, AppError> {
    let mut builder = reqwest::Client::builder().default_headers(headers.clone());
    if let Some(timeout) = timeout_ms {
        builder = builder.timeout(Duration::from_millis(timeout));
    }
    if let Some(connect_timeout) = connect_timeout_ms {
        builder = builder.connect_timeout(Duration::from_millis(connect_timeout));
    }
    builder
        .build()
        .map_err(|err| AppError::new("http_client", format!("Failed to build client: {err}")))
}

pub(crate) fn split_headers(headers: &[Header]) -> Result<(HeaderMap, Option<String>), AppError> {
    let mut header_map = HeaderMap::new();
    let mut auth_token = None;

    for header in headers {
        if header.name.eq_ignore_ascii_case("authorization")
            && let Some(token) = parse_bearer_token(&header.value)
        {
            if auth_token.is_none() {
                auth_token = Some(token);
            }
            continue;
        }

        let name = HeaderName::from_bytes(header.name.as_bytes()).map_err(|err| {
            AppError::new(
                "invalid_header",
                format!("Invalid header name {}: {err}", header.name),
            )
        })?;
        let value = HeaderValue::from_str(&header.value).map_err(|err| {
            AppError::new(
                "invalid_header",
                format!("Invalid header value {}: {err}", header.name),
            )
        })?;
        header_map.append(name, value);
    }

    Ok((header_map, auth_token))
}

fn parse_bearer_token(value: &str) -> Option<String> {
    let trimmed = value.trim();
    let lower = trimmed.to_ascii_lowercase();
    if lower.starts_with("bearer ") {
        let token = trimmed[7..].trim();
        if !token.is_empty() {
            return Some(token.to_string());
        }
    }
    None
}

pub(crate) fn build_client_info(connection: &ResolvedConnection) -> ClientInfo {
    let mut info = ClientInfo::default();
    if let Some(name) = &connection.client_name {
        info.client_info.name = name.clone();
    }
    if let Some(version) = &connection.client_version {
        info.client_info.version = version.clone();
    }
    info
}
