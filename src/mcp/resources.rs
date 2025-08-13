use crate::mcp::types::*;
use rpc_router::{HandlerResult, IntoHandlerError};
use serde_json::json;
use std::fs;
use url::Url;

pub async fn resources_list(
    _request: Option<ListResourcesRequest>,
) -> HandlerResult<ListResourcesResult> {
    let response = ListResourcesResult {
        resources: vec![Resource {
            uri: Url::parse("elm://elm.json").unwrap(),
            name: "elm.json".to_string(),
            description: Some("Project's elm.json file".to_string()),
            mime_type: Some("application/json".to_string()),
        }],
        next_cursor: None,
    };
    Ok(response)
}

pub async fn resource_read(request: ReadResourceRequest) -> HandlerResult<ReadResourceResult> {
    if request.uri.as_str() == "elm://elm.json" {
        // Try to find elm.json in the current directory or parent directories
        let elm_json_path = find_elm_json()
            .map_err(|e| json!({"code": -32603, "message": e}).into_handler_error())?;

        let content = fs::read_to_string(&elm_json_path).map_err(|e| {
            json!({"code": -32603, "message": format!("Failed to read elm.json: {}", e)})
                .into_handler_error()
        })?;

        let response = ReadResourceResult {
            content: ResourceContent {
                uri: request.uri.clone(),
                mime_type: Some("application/json".to_string()),
                text: Some(content),
                blob: None,
            },
        };
        Ok(response)
    } else {
        Err(
            json!({"code": -32602, "message": format!("Unknown resource: {}", request.uri)})
                .into_handler_error(),
        )
    }
}

fn find_elm_json() -> Result<String, String> {
    let current_dir =
        std::env::current_dir().map_err(|e| format!("Failed to get current directory: {e}"))?;

    let mut dir = current_dir.as_path();

    loop {
        let elm_json_path = dir.join("elm.json");
        if elm_json_path.exists() {
            return Ok(elm_json_path.to_string_lossy().to_string());
        }

        match dir.parent() {
            Some(parent) => dir = parent,
            None => {
                return Err(
                    "elm.json not found in current directory or any parent directory".to_string(),
                )
            }
        }
    }
}
