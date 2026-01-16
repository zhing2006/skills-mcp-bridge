use crate::errors::AppError;
use rmcp::model::{ListResourcesResult, PaginatedRequestParam, Resource};
use serde_json::Value;

use super::McpClient;
use super::util::{json_value, map_service_error};

#[derive(serde::Serialize)]
struct ShortResource {
    name: String,
    uri: String,
    description: Option<String>,
}

impl From<Resource> for ShortResource {
    fn from(resource: Resource) -> Self {
        ShortResource {
            name: resource.name.clone(),
            uri: resource.uri.to_string(),
            description: resource.description.clone(),
        }
    }
}

#[derive(serde::Serialize)]
struct ShortResourcesResult {
    resources: Vec<ShortResource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_cursor: Option<String>,
}

impl McpClient {
    pub async fn list_resources(
        &self,
        cursor: Option<String>,
        name: Option<&str>,
        short: bool,
    ) -> Result<Value, AppError> {
        self.retry("list-resources", || async {
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
                            .list_resources(params)
                            .await
                            .map_err(map_service_error)?;
                        matched.extend(
                            result
                                .resources
                                .into_iter()
                                .filter(|resource| resource.name == name),
                        );
                        match result.next_cursor {
                            Some(cursor) => next = Some(cursor),
                            None => break,
                        }
                    }

                    if matched.is_empty() {
                        Err(AppError::new(
                            "not_found",
                            format!("Resource '{}' not found", name),
                        ))
                    } else if short {
                        json_value(ShortResourcesResult {
                            resources: matched.into_iter().map(ShortResource::from).collect(),
                            next_cursor: None,
                        })
                    } else {
                        json_value(ListResourcesResult::with_all_items(matched))
                    }
                }
                None => {
                    let params = cursor.clone().map(|cursor| PaginatedRequestParam {
                        cursor: Some(cursor),
                    });
                    let result = service
                        .peer()
                        .list_resources(params)
                        .await
                        .map_err(map_service_error)?;
                    if short {
                        json_value(ShortResourcesResult {
                            resources: result
                                .resources
                                .into_iter()
                                .map(ShortResource::from)
                                .collect(),
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
