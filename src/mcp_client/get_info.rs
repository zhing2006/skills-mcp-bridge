use crate::errors::AppError;
use serde_json::Value;

use super::McpClient;
use super::util::json_value;

impl McpClient {
    pub async fn get_info(&self) -> Result<Value, AppError> {
        let info = self.fetch_info().await?;
        json_value(info)
    }
}
