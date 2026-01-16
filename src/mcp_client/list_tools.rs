use crate::errors::AppError;
use rmcp::model::{ListToolsResult, PaginatedRequestParam, Tool};
use serde_json::Value;

use super::McpClient;
use super::util::{json_value, map_service_error};

#[derive(serde::Serialize)]
struct ShortTool {
    name: String,
    description: Option<String>,
}

impl From<Tool> for ShortTool {
    fn from(tool: Tool) -> Self {
        ShortTool {
            name: tool.name.to_string(),
            description: tool.description.map(|s| s.to_string()),
        }
    }
}

#[derive(serde::Serialize)]
struct ShortToolsResult {
    tools: Vec<ShortTool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_cursor: Option<String>,
}

impl McpClient {
    pub async fn list_tools(
        &self,
        cursor: Option<String>,
        name: Option<&str>,
        short: bool,
    ) -> Result<Value, AppError> {
        self.retry("list-tools", || async {
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
                            .list_tools(params)
                            .await
                            .map_err(map_service_error)?;
                        matched.extend(
                            result
                                .tools
                                .into_iter()
                                .filter(|tool| tool.name.as_ref() == name),
                        );
                        match result.next_cursor {
                            Some(cursor) => next = Some(cursor),
                            None => break,
                        }
                    }

                    if matched.is_empty() {
                        Err(AppError::new(
                            "not_found",
                            format!("Tool '{}' not found", name),
                        ))
                    } else if short {
                        json_value(ShortToolsResult {
                            tools: matched.into_iter().map(ShortTool::from).collect(),
                            next_cursor: None,
                        })
                    } else {
                        json_value(ListToolsResult::with_all_items(matched))
                    }
                }
                None => {
                    let params = cursor.clone().map(|cursor| PaginatedRequestParam {
                        cursor: Some(cursor),
                    });
                    let result = service
                        .peer()
                        .list_tools(params)
                        .await
                        .map_err(map_service_error)?;
                    if short {
                        json_value(ShortToolsResult {
                            tools: result.tools.into_iter().map(ShortTool::from).collect(),
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
