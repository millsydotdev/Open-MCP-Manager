"use client";

import { McpServer, McpConfig } from "@/lib/types";
import { X, Copy, Check, Download, Layers, Zap, Info } from "lucide-react";
import { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";

interface ConfigViewerProps {
  servers: McpServer[];
  onClose: () => void;
}

export function ConfigViewer({ servers, onClose }: ConfigViewerProps) {
  const [copied, setCopied] = useState(false);
  const [mode, setMode] = useState<'hub' | 'direct'>('hub');
  const [origin, setOrigin] = useState("");

  useEffect(() => {
    setOrigin(window.location.origin);
  }, []);

  const activeServers = servers.filter((s) => s.is_active);
  
  const directConfig: McpConfig = {
    mcpServers: activeServers.reduce((acc, server) => {
      acc[server.name] = {
        command: server.command,
        args: server.args,
        url: server.url,
        env: Object.keys(server.env).length > 0 ? server.env : undefined,
      };
      return acc;
    }, {} as McpConfig["mcpServers"]),
  };

  const hubConfig: McpConfig = {
    mcpServers: {
      "mcp-hub": {
        url: `${origin}/api/mcp/sse`
      }
    }
  };

  const config = mode === 'hub' ? hubConfig : directConfig;
  const configString = JSON.stringify(config, null, 2);

  const copyToClipboard = () => {
    navigator.clipboard.writeText(configString);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const downloadConfig = () => {
    const blob = new Blob([configString], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "claude_desktop_config.json";
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-md">
      <motion.div
        initial={{ opacity: 0, scale: 0.95, y: 20 }}
        animate={{ opacity: 1, scale: 1, y: 0 }}
        className="w-full max-w-3xl overflow-hidden rounded-[2.5rem] bg-white shadow-2xl dark:bg-zinc-950 dark:border dark:border-zinc-800"
      >
        <div className="flex items-center justify-between border-b border-zinc-100 p-8 dark:border-zinc-900">
          <div>
            <h2 className="text-2xl font-bold text-zinc-900 dark:text-white">Editor Configuration</h2>
            <p className="text-sm text-zinc-500 dark:text-zinc-400">Choose how you want to integrate with your editor.</p>
          </div>
          <button onClick={onClose} className="rounded-full p-2 hover:bg-zinc-100 dark:hover:bg-zinc-900 transition-colors">
            <X className="h-6 w-6 text-zinc-400" />
          </button>
        </div>

        <div className="p-8 space-y-8">
          <div className="flex gap-4 p-1.5 bg-zinc-100 dark:bg-zinc-900 rounded-2xl w-fit mx-auto">
            <button
              onClick={() => setMode('hub')}
              className={`flex items-center gap-2 px-6 py-2.5 text-sm font-bold rounded-xl transition-all ${mode === 'hub' ? 'bg-white dark:bg-zinc-800 shadow-sm text-indigo-600 dark:text-indigo-400' : 'text-zinc-500 hover:text-zinc-700 dark:hover:text-zinc-300'}`}
            >
              <Zap className="h-4 w-4" />
              Hub Mode
            </button>
            <button
              onClick={() => setMode('direct')}
              className={`flex items-center gap-2 px-6 py-2.5 text-sm font-bold rounded-xl transition-all ${mode === 'direct' ? 'bg-white dark:bg-zinc-800 shadow-sm text-indigo-600 dark:text-indigo-400' : 'text-zinc-500 hover:text-zinc-700 dark:hover:text-zinc-300'}`}
            >
              <Layers className="h-4 w-4" />
              Direct Mode
            </button>
          </div>

          <AnimatePresence mode="wait">
            <motion.div
              key={mode}
              initial={{ opacity: 0, x: mode === 'hub' ? -20 : 20 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: mode === 'hub' ? 20 : -20 }}
              className="space-y-4"
            >
              <div className="flex items-start gap-4 p-4 rounded-2xl bg-indigo-50/50 dark:bg-indigo-500/5 border border-indigo-100 dark:border-indigo-500/10">
                <Info className="h-5 w-5 text-indigo-500 mt-0.5 shrink-0" />
                <p className="text-sm text-indigo-700 dark:text-indigo-300 leading-relaxed">
                  {mode === 'hub' 
                    ? "Connects your editor to this manager. Changes here are automatically reflected in your editor without manual file updates."
                    : "Generates a complete list of all active servers. You'll need to re-copy this file whenever you add or remove servers."}
                </p>
              </div>

              <div className="relative group">
                <pre className="max-h-[300px] overflow-auto rounded-3xl bg-zinc-950 p-6 text-xs font-mono text-zinc-300 border border-zinc-800 scrollbar-hide">
                  {configString}
                </pre>
                <div className="absolute right-4 top-4 flex gap-2">
                  <button
                    onClick={copyToClipboard}
                    className="rounded-xl bg-zinc-800 p-3 text-zinc-400 hover:bg-zinc-700 hover:text-white transition-all active:scale-95"
                    title="Copy to clipboard"
                  >
                    {copied ? <Check className="h-5 w-5 text-green-500" /> : <Copy className="h-5 w-5" />}
                  </button>
                  <button
                    onClick={downloadConfig}
                    className="rounded-xl bg-zinc-800 p-3 text-zinc-400 hover:bg-zinc-700 hover:text-white transition-all active:scale-95"
                    title="Download JSON"
                  >
                    <Download className="h-5 w-5" />
                  </button>
                </div>
              </div>
            </motion.div>
          </AnimatePresence>

          <div className="grid grid-cols-2 gap-4">
            <div className="p-5 rounded-3xl bg-zinc-50 dark:bg-zinc-900/50 border dark:border-zinc-900">
              <h4 className="text-xs font-bold uppercase tracking-widest text-zinc-400 dark:text-zinc-500 mb-3">macOS Location</h4>
              <code className="text-[11px] font-mono text-zinc-600 dark:text-zinc-300 break-all leading-relaxed">
                ~/Library/Application Support/Claude/claude_desktop_config.json
              </code>
            </div>
            <div className="p-5 rounded-3xl bg-zinc-50 dark:bg-zinc-900/50 border dark:border-zinc-900">
              <h4 className="text-xs font-bold uppercase tracking-widest text-zinc-400 dark:text-zinc-500 mb-3">Windows Location</h4>
              <code className="text-[11px] font-mono text-zinc-600 dark:text-zinc-300 break-all leading-relaxed">
                %APPDATA%\Claude\claude_desktop_config.json
              </code>
            </div>
          </div>
        </div>
      </motion.div>
    </div>
  );
}
