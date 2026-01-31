export interface McpServer {
  id: string;
  name: string;
  type: 'stdio' | 'sse';
  command?: string;
  args?: string[];
  url?: string;
  env: Record<string, string>;
  description: string | null;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface McpConfig {
  mcpServers: Record<string, {
    command?: string;
    args?: string[];
    url?: string;
    env?: Record<string, string>;
  }>;
}
