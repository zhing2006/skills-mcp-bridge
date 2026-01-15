use crate::errors::AppError;
use rmcp::model::{ListPromptsResult, PaginatedRequestParam};
use serde_json::Value;

use super::McpClient;
use super::util::{json_value, map_service_error};

impl McpClient {
    pub async fn list_prompts(
        &self,
        cursor: Option<String>,
        name: Option<&str>,
    ) -> Result<Value, AppError> {
        self.retry("list-prompts", || async {
            let service = self.connect(false).await?;
            let output = match name {
                Some(name) => {
                    let mut next = cursor.clone();
                    let mut matched = Vec::new();

                    loop {
                        let params = next.clone().map(|cursor| PaginatedRequestParam {
                            cursor: Some(cursor),
                        });
                        let result = service
                            .peer()
                            .list_prompts(params)
                            .await
                            .map_err(map_service_error)?;
                        matched.extend(
                            result
                                .prompts
                                .into_iter()
                                .filter(|prompt| prompt.name == name),
                        );
                        match result.next_cursor {
                            Some(cursor) => next = Some(cursor),
                            None => break,
                        }
                    }

                    if matched.is_empty() {
                        Err(AppError::new(
                            "not_found",
                            format!("Prompt '{}' not found", name),
                        ))
                    } else {
                        json_value(ListPromptsResult::with_all_items(matched))
                    }
                }
                None => {
                    let params = cursor.clone().map(|cursor| PaginatedRequestParam {
                        cursor: Some(cursor),
                    });
                    let result = service
                        .peer()
                        .list_prompts(params)
                        .await
                        .map_err(map_service_error)?;
                    json_value(result)
                }
            };
            let _ = service.cancel().await;
            output
        })
        .await
    }
}
