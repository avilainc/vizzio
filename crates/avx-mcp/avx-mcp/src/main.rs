use avila_error::Result;
use avx_mcp::{McpServer, get_all_resources, get_all_tools};

fn main() -> Result<()> {
    // Initialize basic logging to stderr
    eprintln!("🚀 AVX MCP Server initializing...");

    // Create and configure server
    let mut server = McpServer::new();

    // Register all resources
    for resource in get_all_resources() {
        server.register_resource(resource);
    }

    // Register all tools
    for tool in get_all_tools() {
        server.register_tool(tool);
    }

    // Run server
    server.run()
}
