use crate::errors::AppError;
use rmcp::model::PaginatedRequestParam;
use serde_json::Value;

use super::McpClient;
use super::util::{json_value, map_service_error};

impl McpClient {
    pub async fn list_prompts(&self, cursor: Option<String>) -> Result<Value, AppError> {
        self.retry("list-prompts", || async {
            let service = self.connect(false).await?;
            let params = cursor.clone().map(|cursor| PaginatedRequestParam {
                cursor: Some(cursor),
            });
            let result = service
                .peer()
                .list_prompts(params)
                .await
                .map_err(map_service_error)?;
            let _ = service.cancel().await;
            json_value(result)
        })
        .await
    }
}
