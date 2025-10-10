use crate::mcp::types::*;
use rpc_router::{HandlerResult, IntoHandlerError};
use serde_json::json;

pub async fn prompts_list(
    _request: Option<ListPromptsRequest>,
) -> HandlerResult<ListPromptsResult> {
    let response = ListPromptsResult {
        next_cursor: None,
        prompts: vec![
            Prompt {
                name: "analyze-dependencies".to_string(),
                description: Some("Analyze your Elm project's dependencies, explaining what each package does and suggesting optimizations. Proactively use when user asks about their elm.json or project structure.".to_string()),
                arguments: None,
            },
            Prompt {
                name: "explore-package".to_string(),
                description: Some("Explore the capabilities of a specific Elm package by examining its exports, modules, and key functions. Use when user mentions a package name or asks 'what can I do with X package'.".to_string()),
                arguments: Some(vec![
                    PromptArgument {
                        name: "package".to_string(),
                        description: Some("Package name in format 'author/name' (e.g., 'elm/core')".to_string()),
                        required: Some(true),
                    },
                ]),
            },
            Prompt {
                name: "find-function".to_string(),
                description: Some("Search for functions across your Elm dependencies that match a specific capability or use case. Proactively use when user asks 'how do I do X' in Elm.".to_string()),
                arguments: Some(vec![
                    PromptArgument {
                        name: "capability".to_string(),
                        description: Some("What the user wants to accomplish (e.g., 'parse JSON', 'map over a list', 'handle HTTP errors')".to_string()),
                        required: Some(true),
                    },
                ]),
            },
            Prompt {
                name: "debug-import".to_string(),
                description: Some("Explain what functions and types are available from a specific Elm module import. Use when user has import errors or asks about available functions from an import.".to_string()),
                arguments: Some(vec![
                    PromptArgument {
                        name: "module_path".to_string(),
                        description: Some("Full module path (e.g., 'List', 'Html.Attributes', 'Json.Decode')".to_string()),
                        required: Some(true),
                    },
                ]),
            },
            Prompt {
                name: "package-comparison".to_string(),
                description: Some("Compare two Elm packages to help choose the best one for a specific use case. Use when user is deciding between alternatives.".to_string()),
                arguments: Some(vec![
                    PromptArgument {
                        name: "package1".to_string(),
                        description: Some("First package in format 'author/name'".to_string()),
                        required: Some(true),
                    },
                    PromptArgument {
                        name: "package2".to_string(),
                        description: Some("Second package in format 'author/name'".to_string()),
                        required: Some(true),
                    },
                ]),
            },
        ],
    };
    Ok(response)
}

pub async fn prompts_get(request: GetPromptRequest) -> HandlerResult<PromptResult> {
    match request.name.as_str() {
        "analyze-dependencies" => {
            Ok(PromptResult {
                description: "Analyze Elm project dependencies and provide insights".to_string(),
                messages: Some(vec![
                    PromptMessage {
                        role: "user".to_string(),
                        content: PromptMessageContent {
                            type_name: "text".to_string(),
                            text: "Please analyze my Elm project's dependencies. First, list all packages from elm.json, then for each direct dependency, fetch its README to understand what it does. Provide a summary of: 1) What packages are used, 2) What each package's main purpose is, 3) Any potential concerns or suggestions.".to_string(),
                        },
                    },
                ]),
            })
        }
        "explore-package" => {
            let package = request
                .arguments
                .as_ref()
                .and_then(|args| args.get("package"))
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    json!({"code": -32602, "message": "Missing required argument 'package'"})
                        .into_handler_error()
                })?;

            // Parse package into author/name
            let parts: Vec<&str> = package.split('/').collect();
            if parts.len() != 2 {
                return Err(json!({"code": -32602, "message": "Package must be in format 'author/name'"}).into_handler_error());
            }

            Ok(PromptResult {
                description: format!("Explore the {} package", package),
                messages: Some(vec![
                    PromptMessage {
                        role: "user".to_string(),
                        content: PromptMessageContent {
                            type_name: "text".to_string(),
                            text: format!(
                                "Please explore the '{}' package. First, check if it's in my elm.json dependencies. Then fetch its README and exports. Provide: 1) Overview of what the package does, 2) Key modules and their purposes, 3) Most commonly used functions with examples.",
                                package
                            ),
                        },
                    },
                ]),
            })
        }
        "find-function" => {
            let capability = request
                .arguments
                .as_ref()
                .and_then(|args| args.get("capability"))
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    json!({"code": -32602, "message": "Missing required argument 'capability'"})
                        .into_handler_error()
                })?;

            Ok(PromptResult {
                description: format!("Find functions for: {}", capability),
                messages: Some(vec![
                    PromptMessage {
                        role: "user".to_string(),
                        content: PromptMessageContent {
                            type_name: "text".to_string(),
                            text: format!(
                                "I need to '{}' in Elm. Please search through my project's dependencies to find relevant functions. List all packages, then explore the most likely packages (like elm/core, elm/json, etc.) to find functions that could help. Provide function names, type signatures, and usage examples.",
                                capability
                            ),
                        },
                    },
                ]),
            })
        }
        "debug-import" => {
            let module_path = request
                .arguments
                .as_ref()
                .and_then(|args| args.get("module_path"))
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    json!({"code": -32602, "message": "Missing required argument 'module_path'"})
                        .into_handler_error()
                })?;

            Ok(PromptResult {
                description: format!("Debug import for module: {}", module_path),
                messages: Some(vec![
                    PromptMessage {
                        role: "user".to_string(),
                        content: PromptMessageContent {
                            type_name: "text".to_string(),
                            text: format!(
                                "I'm trying to use the '{}' module in Elm. Please help me understand what's available. First, determine which package provides this module by checking my dependencies. Then fetch the exports for this specific module and explain: 1) All available functions with their type signatures, 2) Common usage patterns, 3) What I can import from this module.",
                                module_path
                            ),
                        },
                    },
                ]),
            })
        }
        "package-comparison" => {
            let package1 = request
                .arguments
                .as_ref()
                .and_then(|args| args.get("package1"))
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    json!({"code": -32602, "message": "Missing required argument 'package1'"})
                        .into_handler_error()
                })?;

            let package2 = request
                .arguments
                .as_ref()
                .and_then(|args| args.get("package2"))
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    json!({"code": -32602, "message": "Missing required argument 'package2'"})
                        .into_handler_error()
                })?;

            Ok(PromptResult {
                description: format!("Compare {} vs {}", package1, package2),
                messages: Some(vec![
                    PromptMessage {
                        role: "user".to_string(),
                        content: PromptMessageContent {
                            type_name: "text".to_string(),
                            text: format!(
                                "Please compare the '{}' and '{}' packages. For each package, fetch the README and exports. Then provide a comparison covering: 1) Main purpose and use cases, 2) API differences and complexity, 3) Which one I should choose and why, 4) Are either already in my elm.json?",
                                package1, package2
                            ),
                        },
                    },
                ]),
            })
        }
        _ => Err(
            json!({"code": -32602, "message": format!("Prompt '{}' not found", request.name)})
                .into_handler_error(),
        ),
    }
}
