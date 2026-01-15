use crate::errors::AppError;
use rmcp::model::{ListResourcesResult, PaginatedRequestParam};
use serde_json::Value;

use super::McpClient;
use super::util::{json_value, map_service_error};

impl McpClient {
    pub async fn list_resources(
        &self,
        cursor: Option<String>,
        name: Option<&str>,
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
                    json_value(result)
                }
            };
            let _ = service.cancel().await;
            output
        })
        .await
    }
}
