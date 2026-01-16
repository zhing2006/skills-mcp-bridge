use crate::errors::AppError;
use rmcp::model::{ListPromptsResult, PaginatedRequestParam, Prompt};
use serde_json::Value;

use super::McpClient;
use super::util::{json_value, map_service_error};

#[derive(serde::Serialize)]
struct ShortPrompt {
    name: String,
    description: Option<String>,
}

impl From<Prompt> for ShortPrompt {
    fn from(prompt: Prompt) -> Self {
        ShortPrompt {
            name: prompt.name,
            description: prompt.description,
        }
    }
}

#[derive(serde::Serialize)]
struct ShortPromptsResult {
    prompts: Vec<ShortPrompt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_cursor: Option<String>,
}

impl McpClient {
    pub async fn list_prompts(
        &self,
        cursor: Option<String>,
        name: Option<&str>,
        short: bool,
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
                    } else if short {
                        json_value(ShortPromptsResult {
                            prompts: matched.into_iter().map(ShortPrompt::from).collect(),
                            next_cursor: None,
                        })
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
                    if short {
                        json_value(ShortPromptsResult {
                            prompts: result.prompts.into_iter().map(ShortPrompt::from).collect(),
                            next_cursor: result.next_cursor,
                        })
                    } else {
                        json_value(result)
                    }
                }
            };
            let _ = service.cancel().await;
            output
        })
        .await
    }
}
