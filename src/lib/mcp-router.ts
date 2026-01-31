import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js";
import { SSEClientTransport } from "@modelcontextprotocol/sdk/client/sse.js";
import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { 
  CallToolRequestSchema, 
  ListToolsRequestSchema,
  ListResourcesRequestSchema,
  ReadResourceRequestSchema,
  ListPromptsRequestSchema,
  GetPromptRequestSchema
} from "@modelcontextprotocol/sdk/types.js";
import { McpServer } from "./types";
import { supabase } from "./supabase";
import EventSource from "eventsource";

// @ts-ignore
global.EventSource = EventSource;

export class McpRouter {
  private server: Server;
  private childClients: Map<string, Client> = new Map();

  constructor() {
    this.server = new Server(
      { name: "mcp-hub", version: "1.1.0" },
      { 
        capabilities: { 
          tools: {},
          resources: {},
          prompts: {}
        } 
      }
    );
    this.setupHandlers();
  }

  private async getActiveServers(): Promise<McpServer[]> {
    const { data, error } = await supabase
      .from("mcp_servers")
      .select("*")
      .eq("is_active", true);
    
    if (error) throw error;
    return data || [];
  }

  private async getChildClient(mcpServer: McpServer): Promise<Client> {
    if (this.childClients.has(mcpServer.id)) {
      return this.childClients.get(mcpServer.id)!;
    }

    const client = new Client(
      { name: "mcp-hub-client", version: "1.1.0" },
      { capabilities: {} }
    );

    let transport;
    if (mcpServer.type === 'sse') {
      transport = new SSEClientTransport(new URL(mcpServer.url!));
    } else {
      transport = new StdioClientTransport({
        command: mcpServer.command!,
        args: mcpServer.args || [],
        env: { ...process.env, ...mcpServer.env } as any
      });
    }

    await client.connect(transport);
    this.childClients.set(mcpServer.id, client);
    return client;
  }

  private setupHandlers() {
    // TOOLS
    this.server.setRequestHandler(ListToolsRequestSchema, async () => {
      const activeServers = await this.getActiveServers();
      const allTools = [];

      for (const s of activeServers) {
        try {
          const client = await this.getChildClient(s);
          const response = await client.request(ListToolsRequestSchema, {});
          const namespacedTools = (response.tools || []).map(tool => ({
            ...tool,
            name: `${s.name}__${tool.name}`,
            description: `[${s.name}] ${tool.description}`
          }));
          allTools.push(...namespacedTools);
        } catch (error) {
          console.error(`Failed to fetch tools from ${s.name}:`, error);
        }
      }

      return { tools: allTools };
    });

    this.server.setRequestHandler(CallToolRequestSchema, async (request) => {
      const [serverName, ...toolNameParts] = request.params.name.split("__");
      const toolName = toolNameParts.join("__");

      const activeServers = await this.getActiveServers();
      const targetServer = activeServers.find(s => s.name === serverName);

      if (!targetServer) {
        throw new Error(`Server ${serverName} not found or inactive`);
      }

      const client = await this.getChildClient(targetServer);
      return await client.request(CallToolRequestSchema, {
        name: toolName,
        arguments: request.params.arguments
      });
    });

    // RESOURCES
    this.server.setRequestHandler(ListResourcesRequestSchema, async () => {
      const activeServers = await this.getActiveServers();
      const allResources = [];

      for (const s of activeServers) {
        try {
          const client = await this.getChildClient(s);
          const response = await client.request(ListResourcesRequestSchema, {});
          const namespacedResources = (response.resources || []).map(resource => ({
            ...resource,
            uri: `mcp://${s.name}/${resource.uri}`,
            name: `[${s.name}] ${resource.name}`
          }));
          allResources.push(...namespacedResources);
        } catch (error) {
          console.error(`Failed to fetch resources from ${s.name}:`, error);
        }
      }

      return { resources: allResources };
    });

    this.server.setRequestHandler(ReadResourceRequestSchema, async (request) => {
      const url = new URL(request.params.uri);
      const serverName = url.host;
      const childUri = request.params.uri.replace(`mcp://${serverName}/`, "");

      const activeServers = await this.getActiveServers();
      const targetServer = activeServers.find(s => s.name === serverName);

      if (!targetServer) throw new Error(`Server ${serverName} not found`);

      const client = await this.getChildClient(targetServer);
      return await client.request(ReadResourceRequestSchema, {
        uri: childUri
      });
    });

    // PROMPTS
    this.server.setRequestHandler(ListPromptsRequestSchema, async () => {
      const activeServers = await this.getActiveServers();
      const allPrompts = [];

      for (const s of activeServers) {
        try {
          const client = await this.getChildClient(s);
          const response = await client.request(ListPromptsRequestSchema, {});
          const namespacedPrompts = (response.prompts || []).map(prompt => ({
            ...prompt,
            name: `${s.name}__${prompt.name}`,
            description: `[${s.name}] ${prompt.description}`
          }));
          allPrompts.push(...namespacedPrompts);
        } catch (error) {
          console.error(`Failed to fetch prompts from ${s.name}:`, error);
        }
      }

      return { prompts: allPrompts };
    });

    this.server.setRequestHandler(GetPromptRequestSchema, async (request) => {
      const [serverName, ...promptNameParts] = request.params.name.split("__");
      const promptName = promptNameParts.join("__");

      const activeServers = await this.getActiveServers();
      const targetServer = activeServers.find(s => s.name === serverName);

      if (!targetServer) throw new Error(`Server ${serverName} not found`);

      const client = await this.getChildClient(targetServer);
      return await client.request(GetPromptRequestSchema, {
        name: promptName,
        arguments: request.params.arguments
      });
    });
  }

  getMcpServer() {
    return this.server;
  }
}
