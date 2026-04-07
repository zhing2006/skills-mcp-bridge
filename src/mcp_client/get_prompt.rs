use crate::errors::AppError;
use rmcp::model::GetPromptRequestParams;
use serde_json::Value;

use super::McpClient;
use super::text::prompt_messages_to_text;
use super::util::{map_service_error, value_to_object};

impl McpClient {
    pub async fn get_prompt(
        &self,
        prompt_id: String,
        params: Option<Value>,
    ) -> Result<String, AppError> {
        self.retry("get-prompt", || {
            let prompt_id = prompt_id.clone();
            let params = params.clone();
            async move {
                let service = self.connect(false).await?;
                let arguments = value_to_object(params, "params")?;
                let result = service
                    .peer()
                    .get_prompt({
                        let mut params = GetPromptRequestParams::new(prompt_id);
                        if let Some(args) = arguments {
                            params = params.with_arguments(args);
                        }
                        params
                    })
                    .await
                    .map_err(map_service_error)?;
                let _ = service.cancel().await;
                let text = prompt_messages_to_text(&result.messages);
                Ok(text)
            }
        })
        .await
    }
}
