use crate::errors::AppError;
use rmcp::model::JsonObject;
use serde_json::Value;

pub(crate) fn value_to_object(
    value: Option<Value>,
    label: &str,
) -> Result<Option<JsonObject>, AppError> {
    let Some(value) = value else {
        return Ok(None);
    };

    match value {
        Value::Object(map) => Ok(Some(map)),
        _ => Err(AppError::new(
            "invalid_params",
            format!("{label} must be a JSON object"),
        )),
    }
}

pub(crate) fn json_value<T: serde::Serialize>(value: T) -> Result<Value, AppError> {
    serde_json::to_value(value)
        .map_err(|err| AppError::new("json_encode", format!("Failed to encode result: {err}")))
}

pub(crate) fn map_service_error(err: rmcp::service::ServiceError) -> AppError {
    AppError::new("mcp_service", err.to_string())
}

pub(crate) fn map_init_error(err: rmcp::service::ClientInitializeError) -> AppError {
    AppError::new("mcp_init", err.to_string())
}
