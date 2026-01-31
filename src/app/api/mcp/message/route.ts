import { transports } from "@/lib/mcp-transports";
import { NextRequest, NextResponse } from "next/server";

export async function POST(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const sessionId = searchParams.get("sessionId");

  if (!sessionId) {
    return NextResponse.json({ error: "Missing sessionId" }, { status: 400 });
  }

  const transport = transports.get(sessionId);
  if (!transport) {
    return NextResponse.json({ error: "Session not found" }, { status: 404 });
  }

  await transport.handlePostMessage(request as any, new Response());
  return new NextResponse(null, { status: 202 });
}
