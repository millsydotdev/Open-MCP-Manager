import { SSEServerTransport } from "@modelcontextprotocol/sdk/server/sse.js";
import { McpRouter } from "@/lib/mcp-router";
import { transports } from "@/lib/mcp-transports";
import { NextResponse } from "next/server";

export async function GET() {
  const router = new McpRouter();
  const sessionId = Math.random().toString(36).substring(7);
  
  // Create a base response to initialize the transport
  const baseResponse = new Response();
  
  const transport = new SSEServerTransport(
    `/api/mcp/message?sessionId=${sessionId}`,
    baseResponse
  );

  transports.set(sessionId, transport);

  await router.getMcpServer().connect(transport);

  // Use the transport to handle the SSE response
  // @ts-ignore
  return await transport.handleSSE();
}
