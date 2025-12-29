use crate::elm::{fetcher, reader, search, PackageInfo};
use crate::mcp::types::*;
use maplit::hashmap;
use rpc_router::{Handler, HandlerResult, IntoHandlerError, RouterBuilder, RpcParams};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::env;

const DEPRECATION_WARNING: &str = "⚠️ DEPRECATED: This MCP server is deprecated. Use the `migrate-to-skills` prompt for migration instructions, or install the new plugin: /plugin marketplace add caseyWebb/elm-claude-plugin\n\n";

/// register all tools to the router
pub fn register_tools(router_builder: RouterBuilder) -> RouterBuilder {
    router_builder
        .append_dyn("tools/list", tools_list.into_dyn())
        .append_dyn("list_installed_packages", list_installed.into_dyn())
        .append_dyn("search_packages", search_packages.into_dyn())
        .append_dyn("get_elm_package_readme", get_readme.into_dyn())
        .append_dyn("get_elm_package_exports", get_exports.into_dyn())
        .append_dyn("get_elm_package_export_docs", get_export_docs.into_dyn())
}

pub async fn tools_list(_request: Option<ListToolsRequest>) -> HandlerResult<ListToolsResult> {
    let response = ListToolsResult {
        tools: vec![
            Tool {
                name: "list_installed_packages".to_string(),
                description: Some("List all Elm packages from elm.json file. Returns direct and indirect dependencies with their versions.\n\n**Use this when:** User asks about dependencies in their current project.\n\n**Next steps:** After listing packages, use get_elm_package_readme for overview or get_elm_package_exports to browse available functions.".to_string()),
                input_schema: ToolInputSchema {
                    type_name: "object".to_string(),
                    properties: hashmap! {
                        "include_indirect".to_string() => ToolInputSchemaProperty {
                            type_name: Some("boolean".to_string()),
                            description: Some("Include indirect dependencies (default: false). Set to true when doing comprehensive dependency analysis.".to_string()),
                            enum_values: None,
                        }
                    },
                    required: vec![],
                },
            },
            Tool {
                name: "search_packages".to_string(),
                description: Some("Search the Elm package registry for packages matching a query. Uses fuzzy matching on package names and descriptions. Perfect for discovering new packages.\n\n**Use this when:** User asks 'find me a package that does X', 'what packages are available for Y', or wants to explore alternatives before installation.\n\n**Proactive usage:** When user describes a need that might require a new package, search first before suggesting solutions.\n\n**Next steps:** After finding packages, use get_elm_package_readme or get_elm_package_exports to explore them further.".to_string()),
                input_schema: ToolInputSchema {
                    type_name: "object".to_string(),
                    properties: hashmap! {
                        "query".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_string()),
                            description: Some("Search query - can be package name, keywords, or description of what you're looking for (e.g., 'json decode', 'http', 'date formatting')".to_string()),
                            enum_values: None,
                        },
                        "already_included".to_string() => ToolInputSchemaProperty {
                            type_name: Some("boolean".to_string()),
                            description: Some("Include packages already in elm.json (default: true). Set to false to only show packages not yet installed, useful for finding alternatives.".to_string()),
                            enum_values: None,
                        }
                    },
                    required: vec!["query".to_string()],
                },
            },
            Tool {
                name: "get_elm_package_readme".to_string(),
                description: Some("Get README documentation for an Elm language package from package.elm-lang.org. Provides high-level overview, usage examples, and package philosophy.\n\n**Use this when:** User asks 'what does package X do', needs to understand package concepts, or wants usage examples.\n\n**Workflow:** First use list_installed_packages to discover available packages and their versions, then call this for the specific package.\n\n**Next steps:** Use get_elm_package_exports to see specific functions, or get_elm_package_export_docs for detailed function documentation.".to_string()),
                input_schema: ToolInputSchema {
                    type_name: "object".to_string(),
                    properties: hashmap! {
                        "author".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_string()),
                            description: Some("Package author (e.g., 'elm'). Get this from list_installed_packages output.".to_string()),
                            enum_values: None,
                        },
                        "name".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_string()),
                            description: Some("Package name (e.g., 'core'). Get this from list_installed_packages output.".to_string()),
                            enum_values: None,
                        },
                        "version".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_string()),
                            description: Some("Package version (e.g., '1.0.5'). Get this from list_installed_packages output to use the exact version in the project.".to_string()),
                            enum_values: None,
                        }
                    },
                    required: vec![
                        "author".to_string(),
                        "name".to_string(),
                        "version".to_string(),
                    ],
                },

            },
            Tool {
                name: "get_elm_package_exports".to_string(),
                description: Some("Get all exports from Elm package modules with their type signatures but WITHOUT comments. More efficient for browsing and discovering available functions. Returns a complete tree of all exports organized by module.\n\n**Use this when:** User asks 'what functions are available', wants to browse a package's API, needs to see type signatures, or is looking for a function that does something specific.\n\n**Workflow:** Use list_installed_packages first to get package versions, optionally check README for context, then call this to see the full API surface.\n\n**Next steps:** Once you find the function/type you need, use get_elm_package_export_docs to get detailed documentation with examples.".to_string()),
                input_schema: ToolInputSchema {
                    type_name: "object".to_string(),
                    properties: hashmap! {
                        "author".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_string()),
                            description: Some("Package author (e.g., 'elm'). Get from list_installed_packages.".to_string()),
                            enum_values: None,
                        },
                        "name".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_string()),
                            description: Some("Package name (e.g., 'core'). Get from list_installed_packages.".to_string()),
                            enum_values: None,
                        },
                        "version".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_string()),
                            description: Some("Package version (e.g., '1.0.5'). Get from list_installed_packages.".to_string()),
                            enum_values: None,
                        },
                        "module".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_string()),
                            description: Some("Optional: Filter to specific module (e.g., 'List', 'Maybe', 'Json.Decode'). Use when user asks about a specific module or import.".to_string()),
                            enum_values: None,
                        }
                    },
                    required: vec![
                        "author".to_string(),
                        "name".to_string(),
                        "version".to_string(),
                    ],
                },
            },
            Tool {
                name: "get_elm_package_export_docs".to_string(),
                description: Some("Get the detailed documentation comment for a specific export (function, type, or alias) in an Elm package module. Returns the doc comment which typically includes description, examples, and usage notes.\n\n**Use this when:** User asks 'what does function X do', 'how do I use X', or needs examples for a specific function. This provides the most detailed information about a single item.\n\n**Workflow:** Use get_elm_package_exports first to discover available functions, then call this for the specific function the user needs.\n\n**This is the final step** in the documentation lookup workflow - it provides the most detailed, specific information.".to_string()),
                input_schema: ToolInputSchema {
                    type_name: "object".to_string(),
                    properties: hashmap! {
                        "author".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_string()),
                            description: Some("Package author (e.g., 'elm'). Get from list_installed_packages.".to_string()),
                            enum_values: None,
                        },
                        "name".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_string()),
                            description: Some("Package name (e.g., 'core'). Get from list_installed_packages.".to_string()),
                            enum_values: None,
                        },
                        "version".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_string()),
                            description: Some("Package version (e.g., '1.0.5'). Get from list_installed_packages.".to_string()),
                            enum_values: None,
                        },
                        "module".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_string()),
                            description: Some("Module name (e.g., 'List', 'Maybe'). Get from get_elm_package_exports output.".to_string()),
                            enum_values: None,
                        },
                        "export_name".to_string() => ToolInputSchemaProperty {
                            type_name: Some("string".to_string()),
                            description: Some("Name of the specific export (e.g., 'map', 'Maybe', 'andThen'). Get from get_elm_package_exports output.".to_string()),
                            enum_values: None,
                        }
                    },
                    required: vec![
                        "author".to_string(),
                        "name".to_string(),
                        "version".to_string(),
                        "module".to_string(),
                        "export_name".to_string(),
                    ],
                },
            },
        ],
        next_cursor: None,
    };
    Ok(response)
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct ListInstalledRequest {
    pub include_indirect: Option<bool>,
}

pub async fn list_installed(request: ListInstalledRequest) -> HandlerResult<CallToolResult> {
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
                "author": p.author,
                "name": p.name,
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
                    "author": p.author,
                    "name": p.name,
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
            text: format!("{}{}", DEPRECATION_WARNING, serde_json::to_string_pretty(&result).unwrap()),
        }],
        is_error: false,
    })
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct GetReadmeRequest {
    pub author: String,
    pub name: String,
    pub version: String,
}

pub async fn get_readme(request: GetReadmeRequest) -> HandlerResult<CallToolResult> {
    let package_info = PackageInfo {
        author: request.author,
        name: request.name,
        version: request.version,
    };

    let readme = fetcher::fetch_readme(&package_info)
        .map_err(|e| json!({"code": -32603, "message": e}).into_handler_error())?;

    Ok(CallToolResult {
        content: vec![CallToolResultContent::Text { text: format!("{}{}", DEPRECATION_WARNING, readme) }],
        is_error: false,
    })
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct GetExportsRequest {
    pub author: String,
    pub name: String,
    pub version: String,
    pub module: Option<String>,
}

pub async fn get_exports(request: GetExportsRequest) -> HandlerResult<CallToolResult> {
    let package_info = PackageInfo {
        author: request.author.clone(),
        name: request.name.clone(),
        version: request.version.clone(),
    };

    let modules = fetcher::fetch_docs(&package_info)
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

    let exports_json = json!({
        "author": request.author,
        "name": request.name,
        "version": request.version,
        "modules": filtered_modules.iter().map(|m| json!({
            "name": m.name,
            "unions": m.unions.iter().map(|u| json!({
                "name": u.name,
                "args": u.args,
                "cases": u.cases
            })).collect::<Vec<_>>(),
            "aliases": m.aliases.iter().map(|a| json!({
                "name": a.name,
                "args": a.args,
                "type": a.type_annotation
            })).collect::<Vec<_>>(),
            "values": m.values.iter().map(|v| json!({
                "name": v.name,
                "type": v.type_annotation
            })).collect::<Vec<_>>(),
            "binops": m.binops.iter().map(|b| json!({
                "name": b.name,
                "type": b.type_annotation,
                "associativity": b.associativity,
                "precedence": b.precedence
            })).collect::<Vec<_>>()
        })).collect::<Vec<_>>()
    });

    Ok(CallToolResult {
        content: vec![CallToolResultContent::Text {
            text: format!("{}{}", DEPRECATION_WARNING, serde_json::to_string_pretty(&exports_json).unwrap()),
        }],
        is_error: false,
    })
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct GetExportDocsRequest {
    pub author: String,
    pub name: String,
    pub version: String,
    pub module: String,
    pub export_name: String,
}

pub async fn get_export_docs(request: GetExportDocsRequest) -> HandlerResult<CallToolResult> {
    let package_info = PackageInfo {
        author: request.author.clone(),
        name: request.name.clone(),
        version: request.version.clone(),
    };

    let modules = fetcher::fetch_docs(&package_info)
        .map_err(|e| json!({"code": -32603, "message": e}).into_handler_error())?;

    // Find the specific module
    let module = modules
        .iter()
        .find(|m| m.name == request.module)
        .ok_or_else(|| {
            json!({"code": -32603, "message": format!("Module '{}' not found", request.module)})
                .into_handler_error()
        })?;

    // Search for the export in unions, aliases, values, and binops
    let mut comment = None;
    let mut export_type = None;
    let mut type_annotation = None;

    // Check unions
    if let Some(union) = module.unions.iter().find(|u| u.name == request.export_name) {
        comment = Some(union.comment.clone());
        export_type = Some("union");
        type_annotation = Some(format!("type {} {}", union.name, union.args.join(" ")));
    }
    // Check aliases
    else if let Some(alias) = module
        .aliases
        .iter()
        .find(|a| a.name == request.export_name)
    {
        comment = Some(alias.comment.clone());
        export_type = Some("alias");
        type_annotation = Some(format!(
            "type alias {} {} = {}",
            alias.name,
            alias.args.join(" "),
            alias.type_annotation
        ));
    }
    // Check values
    else if let Some(value) = module.values.iter().find(|v| v.name == request.export_name) {
        comment = Some(value.comment.clone());
        export_type = Some("value");
        type_annotation = Some(format!("{} : {}", value.name, value.type_annotation));
    }
    // Check binops
    else if let Some(binop) = module.binops.iter().find(|b| b.name == request.export_name) {
        comment = Some(binop.comment.clone());
        export_type = Some("binop");
        type_annotation = Some(format!("({}) : {}", binop.name, binop.type_annotation));
    }

    if let Some(comment_text) = comment {
        let result = json!({
            "author": request.author,
            "name": request.name,
            "version": request.version,
            "module": request.module,
            "export_name": request.export_name,
            "export_type": export_type,
            "type_annotation": type_annotation,
            "comment": comment_text
        });

        Ok(CallToolResult {
            content: vec![CallToolResultContent::Text {
                text: format!("{}{}", DEPRECATION_WARNING, serde_json::to_string_pretty(&result).unwrap()),
            }],
            is_error: false,
        })
    } else {
        Err(json!({
            "code": -32603,
            "message": format!("Export '{}' not found in module '{}'", request.export_name, request.module)
        })
        .into_handler_error())
    }
}

#[derive(Deserialize, Serialize, RpcParams)]
pub struct SearchPackagesRequest {
    pub query: String,
    pub already_included: Option<bool>,
}

pub async fn search_packages(request: SearchPackagesRequest) -> HandlerResult<CallToolResult> {
    // Fetch the search index (run blocking HTTP call in separate thread pool)
    let entries = tokio::task::spawn_blocking(search::fetch_search_index)
        .await
        .map_err(|e| {
            json!({"code": -32603, "message": format!("Task join error: {}", e)})
                .into_handler_error()
        })?
        .map_err(|e| json!({"code": -32603, "message": e}).into_handler_error())?;

    // Determine if we should exclude packages from elm.json
    let already_included = request.already_included.unwrap_or(true);
    let exclude_packages = if !already_included {
        // Get packages from elm.json to exclude
        match find_elm_json() {
            Ok(elm_json_path) => match reader::read_elm_json(&elm_json_path) {
                Ok(elm_json) => {
                    let mut excluded = HashSet::new();
                    for pkg in reader::get_direct_packages(&elm_json) {
                        excluded.insert(format!("{}/{}", pkg.author, pkg.name));
                    }
                    for pkg in reader::get_indirect_packages(&elm_json) {
                        excluded.insert(format!("{}/{}", pkg.author, pkg.name));
                    }
                    Some(excluded)
                }
                Err(_) => None,
            },
            Err(_) => None,
        }
    } else {
        None
    };

    // Perform fuzzy search
    let results = search::fuzzy_search(&request.query, &entries, exclude_packages.as_ref(), 20);

    let result = json!({
        "query": request.query,
        "results": results,
        "count": results.len(),
        "excluded_installed": !already_included
    });

    Ok(CallToolResult {
        content: vec![CallToolResultContent::Text {
            text: format!("{}{}", DEPRECATION_WARNING, serde_json::to_string_pretty(&result).unwrap()),
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
