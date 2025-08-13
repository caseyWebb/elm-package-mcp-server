use crate::elm::{fetcher, reader};
use crate::mcp::types::*;
use maplit::hashmap;
use rpc_router::{Handler, HandlerResult, IntoHandlerError, RouterBuilder, RpcParams};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;

/// register all tools to the router
pub fn register_tools(router_builder: RouterBuilder) -> RouterBuilder {
    router_builder
        .append_dyn("tools/list", tools_list.into_dyn())
        .append_dyn("list_packages", list_packages.into_dyn())
        .append_dyn("get_readme", get_readme.into_dyn())
        .append_dyn("get_docs", get_docs.into_dyn())
}

pub async fn tools_list(_request: Option<ListToolsRequest>) -> HandlerResult<ListToolsResult> {
    let response = ListToolsResult {
        tools: vec![
            Tool {
                name: "list_packages".to_string(),
                description: Some("List all Elm packages from elm.json".to_string()),
                input_schema: ToolInputSchema {
                    type_name: "object".to_string(),
                    properties: hashmap! {
                        "include_indirect".to_string() => ToolInputSchemaProperty {
                            type_name: Some("boolean".to_string()),
                            description: Some("Include indirect dependencies (default: false)".to_string()),
                            enum_values: None,
                        }
                    },
                    required: vec![],
                },
            },
            Tool {
                name: "get_readme".to_string(),
                description: Some("Get README for an Elm package".to_string()),
                input_schema: ToolInputSchema {
                    type_name: "object".to_string(),
                    properties: hashmap! {
                        "package".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_string()),
                            description: Some("Package name (e.g., 'elm/core')".to_string()),
                            enum_values: None,
                        }
                    },
                    required: vec!["package".to_string()],
                },
            },
            Tool {
                name: "get_docs".to_string(),
                description: Some("Get documentation for an Elm package".to_string()),
                input_schema: ToolInputSchema {
                    type_name: "object".to_string(),
                    properties: hashmap! {
                        "package".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_string()),
                            description: Some("Package name (e.g., 'elm/core')".to_string()),
                            enum_values: None,
                        },
                        "module".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_string()),
                            description: Some("Optional module name to filter by".to_string()),
                            enum_values: None,
                        }
                    },
                    required: vec!["package".to_string()],
                },
            },
        ],
        next_cursor: None,
    };
    Ok(response)
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct ListPackagesRequest {
    pub include_indirect: Option<bool>,
}

pub async fn list_packages(request: ListPackagesRequest) -> HandlerResult<CallToolResult> {
    let elm_json_path =
        find_elm_json().map_err(|e| json!({"code": -32603, "message": e}).into_handler_error())?;
    let elm_json = reader::read_elm_json(&elm_json_path)
        .map_err(|e| json!({"code": -32603, "message": e}).into_handler_error())?;

    let include_indirect = request.include_indirect.unwrap_or(false);

    let mut packages = reader::get_direct_packages(&elm_json);
    let mut package_list: Vec<Value> = packages
        .iter()
        .map(|p| {
            json!({
                "name": p.full_name(),
                "version": p.version,
                "type": "direct"
            })
        })
        .collect();

    if include_indirect {
        let indirect_packages = reader::get_indirect_packages(&elm_json);
        let indirect_list: Vec<Value> = indirect_packages
            .iter()
            .map(|p| {
                json!({
                    "name": p.full_name(),
                    "version": p.version,
                    "type": "indirect"
                })
            })
            .collect();
        package_list.extend(indirect_list);
        packages.extend(indirect_packages);
    }

    let result = json!({
        "packages": package_list,
        "total": package_list.len(),
        "direct_count": reader::get_direct_packages(&elm_json).len(),
        "indirect_count": reader::get_indirect_packages(&elm_json).len()
    });

    Ok(CallToolResult {
        content: vec![CallToolResultContent::Text {
            text: serde_json::to_string_pretty(&result).unwrap(),
        }],
        is_error: false,
    })
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct GetReadmeRequest {
    pub package: String,
}

pub async fn get_readme(request: GetReadmeRequest) -> HandlerResult<CallToolResult> {
    let elm_json_path =
        find_elm_json().map_err(|e| json!({"code": -32603, "message": e}).into_handler_error())?;
    let elm_json = reader::read_elm_json(&elm_json_path)
        .map_err(|e| json!({"code": -32603, "message": e}).into_handler_error())?;

    let package_info = reader::find_package(&elm_json, &request.package)
        .ok_or_else(|| json!({"code": -32602, "message": format!("Package '{}' not found in elm.json", request.package)}).into_handler_error())?;

    let readme = fetcher::fetch_readme(&package_info)
        .await
        .map_err(|e| json!({"code": -32603, "message": e}).into_handler_error())?;

    Ok(CallToolResult {
        content: vec![CallToolResultContent::Text { text: readme }],
        is_error: false,
    })
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct GetDocsRequest {
    pub package: String,
    pub module: Option<String>,
}

pub async fn get_docs(request: GetDocsRequest) -> HandlerResult<CallToolResult> {
    let elm_json_path =
        find_elm_json().map_err(|e| json!({"code": -32603, "message": e}).into_handler_error())?;
    let elm_json = reader::read_elm_json(&elm_json_path)
        .map_err(|e| json!({"code": -32603, "message": e}).into_handler_error())?;

    let package_info = reader::find_package(&elm_json, &request.package)
        .ok_or_else(|| json!({"code": -32602, "message": format!("Package '{}' not found in elm.json", request.package)}).into_handler_error())?;

    let modules = fetcher::fetch_docs(&package_info)
        .await
        .map_err(|e| json!({"code": -32603, "message": e}).into_handler_error())?;

    // Filter by module if specified
    let filtered_modules = if let Some(module_name) = request.module {
        modules
            .into_iter()
            .filter(|m| m.name == module_name)
            .collect()
    } else {
        modules
    };

    let docs_json = json!({
        "package": request.package,
        "version": package_info.version,
        "modules": filtered_modules.iter().map(|m| json!({
            "name": m.name,
            "comment": m.comment,
            "unions": m.unions.iter().map(|u| json!({
                "name": u.name,
                "comment": u.comment,
                "args": u.args,
                "cases": u.cases
            })).collect::<Vec<_>>(),
            "aliases": m.aliases.iter().map(|a| json!({
                "name": a.name,
                "comment": a.comment,
                "args": a.args,
                "type": a.type_annotation
            })).collect::<Vec<_>>(),
            "values": m.values.iter().map(|v| json!({
                "name": v.name,
                "type": v.type_annotation,
                "comment": v.comment
            })).collect::<Vec<_>>(),
            "binops": m.binops.iter().map(|b| json!({
                "name": b.name,
                "type": b.type_annotation,
                "comment": b.comment,
                "associativity": b.associativity,
                "precedence": b.precedence
            })).collect::<Vec<_>>()
        })).collect::<Vec<_>>()
    });

    Ok(CallToolResult {
        content: vec![CallToolResultContent::Text {
            text: serde_json::to_string_pretty(&docs_json).unwrap(),
        }],
        is_error: false,
    })
}

fn find_elm_json() -> Result<String, String> {
    let current_dir =
        env::current_dir().map_err(|e| format!("Failed to get current directory: {e}"))?;

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
