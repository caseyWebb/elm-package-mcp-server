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
                "description": "Project's elm.json file containing all dependencies"
            }]);
        }

        if args.prompts {
            output["prompts"] = json!([
                {"name": "analyze-dependencies", "description": "Analyze project dependencies"},
                {"name": "explore-package", "description": "Explore a package's capabilities"},
                {"name": "find-function", "description": "Search for functions by capability"},
                {"name": "debug-import", "description": "Understand module imports"},
                {"name": "discover-packages", "description": "Discover new packages for a specific need"},
                {"name": "package-comparison", "description": "Compare two packages"}
            ]);
        }

        if args.tools {
            output["tools"] = json!([
                {
                    "name": "list_installed_packages",
                    "description": "List all Elm packages from elm.json"
                },
                {
                    "name": "search_packages",
                    "description": "Search the Elm package registry (fuzzy search)"
                },
                {
                    "name": "get_elm_package_readme",
                    "description": "Get README for an Elm package (requires author, name, version)"
                },
                {
                    "name": "get_elm_package_exports",
                    "description": "Get all exports from Elm package modules with type signatures (no comments)"
                },
                {
                    "name": "get_elm_package_export_docs",
                    "description": "Get documentation for a specific export in an Elm package module"
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
            println!("  - analyze-dependencies: Analyze project dependencies");
            println!("  - explore-package: Explore a package's capabilities");
            println!("  - find-function: Search for functions by capability");
            println!("  - debug-import: Understand module imports");
            println!("  - discover-packages: Discover new packages for a specific need");
            println!("  - package-comparison: Compare two packages");
        }

        if args.tools {
            println!("Tools:");
            println!("  - list_installed_packages: List all Elm packages from elm.json");
            println!("  - search_packages: Search the Elm package registry (fuzzy search)");
            println!(
                "  - get_elm_package_readme: Get README for an Elm package (requires author, name, version)"
            );
            println!("  - get_elm_package_exports: Get all exports from Elm package modules with type signatures (no comments)");
            println!("  - get_elm_package_export_docs: Get documentation for a specific export in an Elm package module");
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
        instructions: Some(
            "This server provides tools for working with Elm language packages and documentation lookup.\n\n\
             **Proactively use this server when:**\n\
             - User mentions an Elm package name or asks about Elm packages\n\
             - User has Elm import errors or asks about available functions from a module\n\
             - User is writing Elm code and needs API documentation or guidance\n\
             - User asks 'what does X do' for Elm standard library functions\n\
             - User asks 'how do I do X' in Elm context\n\
             - User is exploring or debugging elm.json dependencies\n\n\
             **Recommended workflow:**\n\
             1. Start with 'list_installed_packages' to discover available packages in the project's elm.json\n\
             2. Use 'get_elm_package_readme' for package overview and main concepts\n\
             3. Use 'get_elm_package_exports' to browse available functions and their type signatures\n\
             4. Use 'get_elm_package_export_docs' to get detailed documentation for specific functions\n\n\
             **Available prompts** (use these for common workflows):\n\
             - 'analyze-dependencies': Analyze project dependencies\n\
             - 'explore-package': Explore a specific package's capabilities\n\
             - 'find-function': Search for functions by capability\n\
             - 'debug-import': Understand what's available from a module\n\
             - 'discover-packages': Discover new packages for a specific need\n\
             - 'package-comparison': Compare two packages"
                .to_string(),
        ),
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
