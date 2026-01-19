use crate::errors::AppError;
use rmcp::model::CallToolRequestParam;
use serde_json::Value;
use std::borrow::Cow;

use super::McpClient;
use super::text::call_tool_result_to_text;
use super::util::{map_service_error, value_to_object};

impl McpClient {
    pub async fn call_tool(&self, tool: String, params: Option<Value>) -> Result<String, AppError> {
        self.retry("call-tool", || {
            let tool = tool.clone();
            let params = params.clone();
            async move {
                let service = self.connect(true).await?;
                let arguments = value_to_object(params, "params")?;
                let result = service
                    .peer()
                    .call_tool(CallToolRequestParam {
                        name: Cow::Owned(tool),
                        arguments,
                        task: None,
                    })
                    .await
                    .map_err(map_service_error)?;
                let _ = service.cancel().await;
                let text = call_tool_result_to_text(&result);
                if result.is_error.unwrap_or(false) {
                    return Err(AppError::new("tool_error", text));
                }
                Ok(text)
            }
        })
        .await
    }
}
