use crate::errors::AppError;
use rmcp::model::ReadResourceRequestParam;

use super::McpClient;
use super::text::resource_contents_to_text;
use super::util::map_service_error;

impl McpClient {
    pub async fn read_resource(&self, uri: String) -> Result<String, AppError> {
        self.retry("read-resource", || {
            let uri = uri.clone();
            async move {
                let service = self.connect(false).await?;
                let result = service
                    .peer()
                    .read_resource(ReadResourceRequestParam { uri })
                    .await
                    .map_err(map_service_error)?;
                let _ = service.cancel().await;
                let text = result
                    .contents
                    .iter()
                    .map(resource_contents_to_text)
                    .collect::<Vec<_>>()
                    .join("\n");
                Ok(text)
            }
        })
        .await
    }
}
