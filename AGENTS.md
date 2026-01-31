## Project Summary
A comprehensive Model Context Protocol (MCP) manager that allows users to manage, configure, and monitor MCP servers. It provides a centralized dashboard and facilitates integration with various editors via a unified MCP Hub.

## Tech Stack
- Framework: Next.js 15+ (App Router)
- Language: TypeScript
- Database: Supabase
- MCP: @modelcontextprotocol/sdk
- Styling: Tailwind CSS
- Motion: Framer Motion
- Icons: Lucide React

## Architecture
- `src/app/api/mcp/sse`: Central MCP Hub endpoint (SSE)
- `src/app/api/mcp/message`: Message handling for the Hub
- `src/lib/mcp-router.ts`: Core logic for aggregating and namespacing child MCP servers
- `src/components`: UI components (ServerForm, ServerCard, Explorer, ConfigViewer)
- `src/lib/types.ts`: Extended MCP types supporting both `stdio` and `sse` transports

## User Preferences
- Modern, distinctive UI with smooth motion
- Dark mode support
- Centralized "One MCP" editor integration

## Project Guidelines
- Support both `stdio` and `sse` MCP server types
- Enable/Disable servers via `is_active` toggle
- Namespace tools in the Hub using `server_name__tool_name` format
- Provide `claude_desktop_config.json` templates in both Hub and Direct modes

## Common Patterns
- Tool namespacing: `[server_name]__[original_tool_name]`
- Hub transport: SSE for editor communication
- Child transport: SSE (remote) or stdio (local)
