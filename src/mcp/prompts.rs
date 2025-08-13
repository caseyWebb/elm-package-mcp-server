use crate::mcp::types::*;
use rpc_router::{HandlerResult, IntoHandlerError};
use serde_json::json;

pub async fn prompts_list(
    _request: Option<ListPromptsRequest>,
) -> HandlerResult<ListPromptsResult> {
    let response = ListPromptsResult {
        next_cursor: None,
        prompts: vec![],
    };
    Ok(response)
}

pub async fn prompts_get(request: GetPromptRequest) -> HandlerResult<PromptResult> {
    Err(
        json!({"code": -32602, "message": format!("Prompt '{}' not found", request.name)})
            .into_handler_error(),
    )
}
