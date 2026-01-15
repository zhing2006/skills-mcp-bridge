use crate::config::ResolvedConnection;
use crate::errors::AppError;
use rmcp::model::ServerInfo;
use rmcp::service::ServiceExt;

use super::connection::{build_backoff, build_client_info, build_http_client, split_headers};
use super::notify::ClientHandlerImpl;
use super::util::map_init_error;

pub struct McpClient {
    connection: ResolvedConnection,
}

#[derive(Debug, Clone, Copy)]
pub enum CapabilityKind {
    Tools,
    Resources,
    Prompts,
}

impl McpClient {
    pub fn new(connection: ResolvedConnection) -> Self {
        Self { connection }
    }

    pub async fn ensure_capability(&self, capability: CapabilityKind) -> Result<(), AppError> {
        let info = self.fetch_info().await?;
        if supports_capability(&info, capability) {
            Ok(())
        } else {
            Err(AppError::new(
                "unsupported_capability",
                format!(
                    "Server does not advertise capability: {}",
                    capability_name(capability)
                ),
            ))
        }
    }

    pub(crate) async fn connect(
        &self,
        emit_notifications: bool,
    ) -> Result<rmcp::service::RunningService<rmcp::service::RoleClient, ClientHandlerImpl>, AppError>
    {
        let (headers, auth_token) = split_headers(&self.connection.headers)?;
        let client = build_http_client(
            &headers,
            self.connection.timeout,
            self.connection.connect_timeout,
        )?;
        let mut config =
            rmcp::transport::streamable_http_client::StreamableHttpClientTransportConfig::with_uri(
                self.connection.url.clone(),
            );
        if let Some(token) = auth_token {
            config = config.auth_header(token);
        }
        let transport = rmcp::transport::StreamableHttpClientTransport::with_client(client, config);
        let handler =
            ClientHandlerImpl::new(build_client_info(&self.connection), emit_notifications);
        handler.serve(transport).await.map_err(map_init_error)
    }

    pub(crate) async fn retry<T, F, Fut>(
        &self,
        _label: &str,
        mut operation: F,
    ) -> Result<T, AppError>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, AppError>>,
    {
        let max_attempts = self.connection.retry.unwrap_or(0).saturating_add(1) as usize;
        let mut attempts = 0usize;
        let backoff = build_backoff(self.connection.retry_backoff);

        backoff::future::retry(backoff, || {
            attempts += 1;
            let attempt = attempts;
            let fut = operation();
            async move {
                match fut.await {
                    Ok(result) => Ok(result),
                    Err(err) => {
                        if attempt >= max_attempts {
                            Err(backoff::Error::permanent(err))
                        } else {
                            Err(backoff::Error::transient(err))
                        }
                    }
                }
            }
        })
        .await
    }

    pub(crate) async fn fetch_info(&self) -> Result<ServerInfo, AppError> {
        self.retry("get-info", || async {
            let service = self.connect(false).await?;
            let info = service
                .peer_info()
                .cloned()
                .ok_or_else(|| AppError::new("missing_server_info", "Server info not available"))?;
            let _ = service.cancel().await;
            Ok(info)
        })
        .await
    }
}

fn supports_capability(info: &ServerInfo, capability: CapabilityKind) -> bool {
    match capability {
        CapabilityKind::Tools => info.capabilities.tools.is_some(),
        CapabilityKind::Resources => info.capabilities.resources.is_some(),
        CapabilityKind::Prompts => info.capabilities.prompts.is_some(),
    }
}

fn capability_name(capability: CapabilityKind) -> &'static str {
    match capability {
        CapabilityKind::Tools => "tools",
        CapabilityKind::Resources => "resources",
        CapabilityKind::Prompts => "prompts",
    }
}
