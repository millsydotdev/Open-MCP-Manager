-- MCP Servers Table
CREATE TABLE IF NOT EXISTS mcp_servers (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL UNIQUE,
  type TEXT NOT NULL CHECK (type IN ('stdio', 'sse')),
  command TEXT,
  args TEXT[],
  url TEXT,
  env JSONB DEFAULT '{}'::jsonb,
  description TEXT,
  is_active BOOLEAN DEFAULT true,
  created_at TIMESTAMPTZ DEFAULT now(),
  updated_at TIMESTAMPTZ DEFAULT now()
);

-- Enable Realtime for this table
ALTER PUBLICATION supabase_realtime ADD TABLE mcp_servers;
