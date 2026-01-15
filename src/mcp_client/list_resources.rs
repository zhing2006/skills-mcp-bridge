use crate::errors::AppError;
use rmcp::model::PaginatedRequestParam;
use serde_json::Value;

use super::McpClient;
use super::util::{json_value, map_service_error};

impl McpClient {
    pub async fn list_resources(&self, cursor: Option<String>) -> Result<Value, AppError> {
        self.retry("list-resources", || async {
            let service = self.connect(false).await?;
            let params = cursor.clone().map(|cursor| PaginatedRequestParam {
                cursor: Some(cursor),
            });
            let result = service
                .peer()
                .list_resources(params)
                .await
                .map_err(map_service_error)?;
            let _ = service.cancel().await;
            json_value(result)
        })
        .await
    }
}
