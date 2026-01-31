"use server";

import { supabase } from "@/lib/supabase";
import { McpServer } from "@/lib/types";
import { revalidatePath } from "next/cache";

const slugify = (text: string) => {
  return text
    .toString()
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .toLowerCase()
    .trim()
    .replace(/\s+/g, '-')
    .replace(/[^\w-]+/g, '')
    .replace(/--+/g, '-');
};

export async function getServers() {
  const { data, error } = await supabase
    .from("mcp_servers")
    .select("*")
    .order("created_at", { ascending: false });

  if (error) throw error;
  return data as McpServer[];
}

export async function createServer(server: Partial<McpServer>) {
  if (server.name) {
    server.name = slugify(server.name);
  }
  
  const { data, error } = await supabase
    .from("mcp_servers")
    .insert([server])
    .select()
    .single();

  if (error) throw error;
  revalidatePath("/");
  return data as McpServer;
}

export async function updateServer(id: string, server: Partial<McpServer>) {
  if (server.name) {
    server.name = slugify(server.name);
  }

  const { data, error } = await supabase
    .from("mcp_servers")
    .update(server)
    .eq("id", id)
    .select()
    .single();

  if (error) throw error;
  revalidatePath("/");
  return data as McpServer;
}

export async function deleteServer(id: string) {
  const { error } = await supabase
    .from("mcp_servers")
    .delete()
    .eq("id", id);

  if (error) throw error;
  revalidatePath("/");
}

export async function toggleServer(id: string, is_active: boolean) {
  const { error } = await supabase
    .from("mcp_servers")
    .update({ is_active, updated_at: new Date().toISOString() })
    .eq("id", id);

  if (error) throw error;
  revalidatePath("/");
}

export async function restartServer(id: string) {
  // In a real local manager, this might signal a process.
  // Here, we update the timestamp to trigger a config refresh.
  const { error } = await supabase
    .from("mcp_servers")
    .update({ updated_at: new Date().toISOString() })
    .eq("id", id);

  if (error) throw error;
  revalidatePath("/");
}

export async function searchRegistry(query: string = "") {
  try {
    const url = `https://registry.modelcontextprotocol.io/v0.1/servers?search=${encodeURIComponent(query)}&limit=50`;
    const response = await fetch(url);
    if (!response.ok) throw new Error("Failed to fetch registry");
    const data = await response.json();
    return data.servers || [];
  } catch (error) {
    console.error("Registry search error:", error);
    // Fallback to v0 if v0.1 fails (though v0.1 is preferred)
    try {
      const url = `https://registry.modelcontextprotocol.io/v0/servers?search=${encodeURIComponent(query)}`;
      const response = await fetch(url);
      const data = await response.json();
      return data.servers || [];
    } catch (e) {
      return [];
    }
  }
}
