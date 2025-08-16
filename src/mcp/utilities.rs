use crate::mcp::types::*;
use crate::mcp::{PROTOCOL_VERSION, SERVER_NAME, SERVER_VERSION};
use clap::Parser;
use rpc_router::HandlerResult;
use serde_json::json;

#[derive(Parser, Debug)]
#[command(name = SERVER_NAME)]
#[command(version = SERVER_VERSION)]
#[command(about = "MCP server for looking up Elm package documentation", long_about = None)]
pub struct Args {
    /// Enable MCP server mode
    #[arg(long)]
    pub mcp: bool,

    /// Display resources
    #[arg(long)]
    pub resources: bool,

    /// Display prompts
    #[arg(long)]
    pub prompts: bool,

    /// Display tools
    #[arg(long)]
    pub tools: bool,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

pub fn display_info(args: &Args) {
    if args.json {
        let mut output = json!({});

        if args.resources {
            output["resources"] = json!([{
                "uri": "elm://elm.json",
                "name": "elm.json",
                "description": "Project's elm.json file"
            }]);
        }

        if args.prompts {
            output["prompts"] = json!([]);
        }

        if args.tools {
            output["tools"] = json!([
                {
                    "name": "list_packages",
                    "description": "List all Elm packages from elm.json"
                },
                {
                    "name": "get_readme",
                    "description": "Get README for an Elm package (requires author, name, version)"
                },
                {
                    "name": "get_docs",
                    "description": "Get documentation for an Elm package (requires author, name, version)"
                }
            ]);
        }

        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    } else {
        if args.resources {
            println!("Resources:");
            println!("  - elm://elm.json: Project's elm.json file");
        }

        if args.prompts {
            println!("Prompts:");
            println!("  (none)");
        }

        if args.tools {
            println!("Tools:");
            println!("  - list_packages: List all Elm packages from elm.json");
            println!(
                "  - get_readme: Get README for an Elm package (requires author, name, version)"
            );
            println!("  - get_docs: Get documentation for an Elm package (requires author, name, version)");
        }
    }
}

/// handler for `initialize` request from client
pub async fn initialize(_request: InitializeRequest) -> HandlerResult<InitializeResult> {
    let result = InitializeResult {
        protocol_version: PROTOCOL_VERSION.to_string(),
        server_info: Implementation {
            name: SERVER_NAME.to_string(),
            version: SERVER_VERSION.to_string(),
        },
        capabilities: ServerCapabilities {
            experimental: None,
            prompts: Some(PromptCapabilities::default()),
            resources: Some(ResourceCapabilities::default()),
            tools: Some(json!({})),
            roots: None,
            sampling: None,
            logging: None,
        },
        instructions: None,
    };
    Ok(result)
}

/// handler for SIGINT by client
pub fn graceful_shutdown() {
    // shutdown server
}

/// handler for `notifications/initialized` from client
pub fn notifications_initialized() {}

/// handler for `notifications/cancelled` from client
pub fn notifications_cancelled(_params: CancelledNotification) {
    // cancel request
}

pub async fn ping(_request: PingRequest) -> HandlerResult<EmptyResult> {
    Ok(EmptyResult {})
}

pub async fn logging_set_level(_request: SetLevelRequest) -> HandlerResult<LoggingResponse> {
    Ok(LoggingResponse {})
}

pub async fn roots_list(_request: Option<ListRootsRequest>) -> HandlerResult<ListRootsResult> {
    let response = ListRootsResult { roots: vec![] };
    Ok(response)
}
