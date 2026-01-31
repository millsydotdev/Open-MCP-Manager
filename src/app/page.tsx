"use client";

import { useState, useEffect } from "react";
import { McpServer } from "@/lib/types";
import { Container } from "@/components/Container";
import { ServerCard } from "@/components/ServerCard";
import { ServerForm } from "@/components/ServerForm";
import { ConfigViewer } from "@/components/ConfigViewer";
import { Explorer } from "@/components/Explorer";
import { Navbar } from "@/components/Navbar";
import { Plus, Loader2, ServerOff, Globe, Info, Copy, Check, Terminal, ExternalLink, Zap } from "lucide-react";
import { toast } from "sonner";
import {
  getServers,
  createServer,
  updateServer,
  deleteServer,
  toggleServer,
  restartServer,
} from "./actions";
import { motion, AnimatePresence } from "framer-motion";

export default function Dashboard() {
  const [servers, setServers] = useState<McpServer[]>([]);
  const [loading, setLoading] = useState(true);
  const [showForm, setShowForm] = useState(false);
  const [showConfig, setShowConfig] = useState(false);
  const [showExplorer, setShowExplorer] = useState(false);
  const [editingServer, setEditingServer] = useState<McpServer | null>(null);
  const [copied, setCopied] = useState(false);
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
    fetchServers();
  }, []);

  const fetchServers = async () => {
    try {
      setLoading(true);
      const data = await getServers();
      setServers(data);
    } catch (error) {
      toast.error("Failed to fetch servers");
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async (serverData: Partial<McpServer>) => {
    try {
      if (editingServer && editingServer.id) {
        await updateServer(editingServer.id, serverData);
        toast.success("Server updated successfully");
      } else {
        await createServer(serverData);
        toast.success("Server added successfully");
      }
      setShowForm(false);
      setEditingServer(null);
      fetchServers();
    } catch (error) {
      toast.error("Failed to save server");
      console.error(error);
    }
  };

  const handleDelete = async (id: string) => {
    if (!confirm("Are you sure you want to delete this server?")) return;
    try {
      await deleteServer(id);
      toast.success("Server deleted");
      fetchServers();
    } catch (error) {
      toast.error("Failed to delete server");
    }
  };

  const handleToggle = async (id: string, active: boolean) => {
    try {
      await toggleServer(id, active);
      setServers(servers.map(s => s.id === id ? { ...s, is_active: active } : s));
      toast.success(`Server ${active ? 'enabled' : 'disabled'}`);
    } catch (error) {
      toast.error("Failed to update status");
    }
  };

  const handleRestart = async (id: string) => {
    try {
      await restartServer(id);
      toast.success("Server restart signal sent");
      fetchServers();
    } catch (error) {
      toast.error("Failed to restart server");
    }
  };

  const handleInstallFromExplorer = (serverData: Partial<McpServer>) => {
    setEditingServer(serverData as McpServer);
    setShowForm(true);
    setShowExplorer(false);
  };

  const hubUrl = mounted ? `${window.location.origin}/api/mcp/sse` : '';
  const hubConfig = JSON.stringify({
    mcpServers: {
      "mcp-manager-hub": {
        url: hubUrl || '/api/mcp/sse'
      }
    }
  }, null, 2);

  const copyToClipboard = () => {
    navigator.clipboard.writeText(hubConfig);
    setCopied(true);
    toast.success("Hub configuration copied to clipboard");
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="min-h-screen bg-[#fafafa] dark:bg-black selection:bg-indigo-500 selection:text-white">
      <Navbar onExport={() => setShowConfig(true)} />
      
      <Container className="py-16">
        {/* Header Section */}
        <div className="mb-16 flex flex-col items-start justify-between gap-8 sm:flex-row sm:items-end">
          <div className="space-y-4">
            <motion.div
              initial={{ opacity: 0, x: -20 }}
              animate={{ opacity: 1, x: 0 }}
              className="inline-flex items-center gap-2 rounded-full bg-indigo-50 px-4 py-1.5 text-xs font-black uppercase tracking-widest text-indigo-600 dark:bg-indigo-500/10 dark:text-indigo-400"
            >
              <Zap className="h-3 w-3 fill-current" />
              Unified MCP Manager
            </motion.div>
            <h1 className="text-6xl font-black tracking-tighter text-zinc-900 dark:text-white">
              MCP Servers
            </h1>
            <p className="max-w-xl text-lg font-medium text-zinc-500 dark:text-zinc-400 leading-relaxed">
              The only dashboard you need to discover, install, and aggregate Model Context Protocol servers.
            </p>
          </div>
          <div className="flex items-center gap-4">
            <button
              onClick={() => setShowExplorer(true)}
              className="flex items-center gap-3 rounded-[24px] border-2 border-zinc-100 bg-white px-8 py-4 text-sm font-black text-zinc-900 shadow-xl shadow-zinc-200/50 transition-all hover:-translate-y-1 hover:border-indigo-500/30 hover:shadow-indigo-500/10 active:scale-95 dark:border-zinc-800 dark:bg-zinc-950 dark:text-white dark:shadow-none dark:hover:border-zinc-700"
            >
              <Globe className="h-5 w-5 text-indigo-500" />
              Registry
            </button>
            <button
              onClick={() => {
                setEditingServer(null);
                setShowForm(true);
              }}
              className="flex items-center gap-3 rounded-[24px] bg-indigo-600 px-8 py-4 text-sm font-black text-white shadow-xl shadow-indigo-500/40 transition-all hover:-translate-y-1 hover:bg-indigo-700 hover:shadow-indigo-500/60 active:scale-95"
            >
              <Plus className="h-5 w-5" />
              Add Server
            </button>
          </div>
        </div>

        {/* Hub Configuration Section */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="mb-16 overflow-hidden rounded-[40px] border-2 border-indigo-100 bg-indigo-50/30 p-10 dark:border-indigo-900/30 dark:bg-indigo-500/5"
        >
          <div className="flex flex-col gap-10 lg:flex-row lg:items-center">
            <div className="flex-1 space-y-6">
              <div className="flex h-14 w-14 items-center justify-center rounded-2xl bg-indigo-600 text-white shadow-lg shadow-indigo-500/30">
                <Info className="h-7 w-7" />
              </div>
              <h2 className="text-3xl font-black tracking-tight text-zinc-900 dark:text-white">
                One MCP to rule them all.
              </h2>
              <p className="text-lg text-zinc-600 dark:text-zinc-400 leading-relaxed">
                Add this single configuration to your <code className="font-mono font-bold text-indigo-600 dark:text-indigo-400 bg-indigo-100 dark:bg-indigo-900/50 px-1.5 py-0.5 rounded">mcp.json</code>. The manager will automatically aggregate all your active servers and namespace their tools for you.
              </p>
              <div className="flex items-center gap-4 pt-4">
                <div className="flex items-center gap-2 rounded-xl bg-white/50 px-4 py-2 text-sm font-bold text-zinc-500 dark:bg-zinc-900/50">
                  <Terminal className="h-4 w-4" />
                  SSE Support Required
                </div>
                <a 
                  href="https://modelcontextprotocol.io" 
                  target="_blank" 
                  className="flex items-center gap-2 text-sm font-bold text-indigo-600 hover:text-indigo-700 dark:text-indigo-400"
                >
                  MCP Docs <ExternalLink className="h-4 w-4" />
                </a>
              </div>
            </div>
            
            <div className="relative w-full lg:w-[450px]">
              <div className="overflow-hidden rounded-3xl border border-zinc-200 bg-white shadow-2xl dark:border-zinc-800 dark:bg-zinc-950">
                <div className="flex items-center justify-between border-b border-zinc-100 px-6 py-4 dark:border-zinc-800">
                  <span className="text-[10px] font-black uppercase tracking-widest text-zinc-400">mcp.json config</span>
                  <button
                    onClick={copyToClipboard}
                    className="flex items-center gap-2 text-xs font-bold text-indigo-600 dark:text-indigo-400 hover:opacity-70 transition-opacity"
                  >
                    {copied ? <Check className="h-4 w-4" /> : <Copy className="h-4 w-4" />}
                    {copied ? 'Copied!' : 'Copy'}
                  </button>
                </div>
                <div className="p-8">
                  <pre className="font-mono text-xs leading-relaxed text-zinc-600 dark:text-zinc-400 overflow-x-auto">
                    {hubConfig}
                  </pre>
                </div>
              </div>
              <div className="absolute -bottom-4 -right-4 h-24 w-24 bg-indigo-500/10 blur-3xl" />
            </div>
          </div>
        </motion.div>

        {/* Servers Grid */}
        {loading ? (
          <div className="flex h-96 items-center justify-center">
            <div className="flex flex-col items-center gap-4">
              <Loader2 className="h-12 w-12 animate-spin text-indigo-600" />
              <p className="text-sm font-bold text-zinc-400 uppercase tracking-widest">Waking up servers...</p>
            </div>
          </div>
        ) : servers.length === 0 ? (
          <motion.div
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            className="flex flex-col items-center justify-center rounded-[48px] border-4 border-dashed border-zinc-100 py-32 text-center dark:border-zinc-900"
          >
            <div className="mb-8 flex h-24 w-24 items-center justify-center rounded-full bg-zinc-50 dark:bg-zinc-900">
              <ServerOff className="h-10 w-10 text-zinc-300 dark:text-zinc-700" />
            </div>
            <h3 className="text-3xl font-black tracking-tight text-zinc-900 dark:text-white">No servers active</h3>
            <p className="mt-4 max-w-sm text-lg font-medium text-zinc-500 dark:text-zinc-400 leading-relaxed">
              Your dashboard is looking a bit empty. Start by adding a server manually or exploring the registry.
            </p>
            <div className="mt-10 flex items-center gap-8">
              <button
                onClick={() => setShowForm(true)}
                className="text-lg font-black text-indigo-600 hover:text-indigo-700 dark:text-indigo-400"
              >
                Create Manually &rarr;
              </button>
              <button
                onClick={() => setShowExplorer(true)}
                className="text-lg font-black text-indigo-600 hover:text-indigo-700 dark:text-indigo-400"
              >
                Browse Registry &rarr;
              </button>
            </div>
          </motion.div>
        ) : (
          <div className="grid gap-8 sm:grid-cols-2 lg:grid-cols-3">
            <AnimatePresence mode="popLayout">
              {servers.map((server) => (
                <ServerCard
                  key={server.id}
                  server={server}
                  onEdit={(s) => {
                    setEditingServer(s);
                    setShowForm(true);
                  }}
                  onDelete={handleDelete}
                  onToggle={handleToggle}
                  onRestart={handleRestart}
                />
              ))}
            </AnimatePresence>
          </div>
        )}
      </Container>

      {/* Overlays */}
      <AnimatePresence>
        {showForm && (
          <ServerForm
            server={editingServer}
            onSave={handleSave}
            onCancel={() => {
              setShowForm(false);
              setEditingServer(null);
            }}
          />
        )}

        {showConfig && (
          <ConfigViewer
            servers={servers}
            onClose={() => setShowConfig(false)}
          />
        )}

        {showExplorer && (
          <Explorer
            onInstall={handleInstallFromExplorer}
            onClose={() => setShowExplorer(false)}
          />
        )}
      </AnimatePresence>
    </div>
  );
}
