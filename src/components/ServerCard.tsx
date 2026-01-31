"use client";

import { McpServer } from "@/lib/types";
import { Terminal, Settings, Trash2, Power, RefreshCw, Globe, Server as ServerIcon, Cpu, Boxes, Layout } from "lucide-react";
import { motion } from "framer-motion";

interface ServerCardProps {
  server: McpServer;
  onEdit: (server: McpServer) => void;
  onDelete: (id: string) => void;
  onToggle: (id: string, active: boolean) => void;
  onRestart: (id: string) => void;
}

export function ServerCard({ server, onEdit, onDelete, onToggle, onRestart }: ServerCardProps) {
  return (
    <motion.div
      layout
      initial={{ opacity: 0, scale: 0.95 }}
      animate={{ opacity: 1, scale: 1 }}
      exit={{ opacity: 0, scale: 0.9 }}
      transition={{ type: "spring", damping: 20, stiffness: 300 }}
      className={`group relative flex flex-col overflow-hidden rounded-[32px] border-2 transition-all hover:shadow-2xl ${
        server.is_active 
          ? 'border-zinc-100 bg-white dark:border-zinc-800 dark:bg-zinc-950 shadow-lg shadow-zinc-200/50 dark:shadow-none' 
          : 'border-zinc-50 bg-zinc-50/30 grayscale opacity-70 dark:border-zinc-900/50 dark:bg-zinc-900/10'
      }`}
    >
      {/* Background Accent */}
      <div className={`absolute -right-12 -top-12 h-48 w-48 rounded-full blur-[80px] transition-opacity ${
        server.is_active ? 'bg-indigo-500/10 dark:bg-indigo-500/5' : 'opacity-0'
      }`} />

      <div className="relative flex-1 p-8">
        <div className="flex items-start justify-between gap-4 mb-8">
          <div className="flex items-center gap-5">
            <div className={`flex h-16 w-16 items-center justify-center rounded-[24px] shadow-inner transition-all duration-500 ${
              server.is_active 
                ? 'bg-gradient-to-br from-indigo-500 to-violet-600 text-white rotate-0' 
                : 'bg-zinc-200 text-zinc-400 dark:bg-zinc-800 dark:text-zinc-600 rotate-12'
            }`}>
              {server.type === 'sse' ? <Globe className="h-8 w-8" /> : <Terminal className="h-8 w-8" />}
            </div>
            <div>
              <div className="flex items-center gap-3">
                <h3 className="text-xl font-black text-zinc-900 dark:text-white tracking-tight">
                  {server.name}
                </h3>
                <span className={`h-2.5 w-2.5 rounded-full ${server.is_active ? 'bg-green-500 shadow-[0_0_12px_rgba(34,197,94,0.6)] animate-pulse' : 'bg-zinc-300 dark:bg-zinc-700'}`} />
              </div>
              <p className="mt-1 text-sm font-medium text-zinc-500 dark:text-zinc-400">
                {server.type === 'sse' ? 'Remote SSE Server' : 'Local Tool Server'}
              </p>
            </div>
          </div>
          
          <div className="flex gap-2">
            <button
              onClick={() => onToggle(server.id, !server.is_active)}
              className={`flex h-11 w-11 items-center justify-center rounded-2xl transition-all active:scale-90 ${
                server.is_active 
                  ? 'bg-green-50 text-green-600 hover:bg-green-100 dark:bg-green-500/10 dark:text-green-400' 
                  : 'bg-zinc-100 text-zinc-400 hover:bg-zinc-200 dark:bg-zinc-900 dark:text-zinc-600'
              }`}
            >
              <Power className="h-5 w-5" />
            </button>
          </div>
        </div>

        <p className="text-sm leading-relaxed text-zinc-600 dark:text-zinc-400 line-clamp-2 mb-8 h-10 italic">
          {server.description || "No description provided for this server."}
        </p>

        <div className="space-y-4">
          <div className="rounded-2xl bg-zinc-50 p-5 dark:bg-zinc-900/50 border border-zinc-100 dark:border-zinc-800/50">
            <div className="flex items-center gap-2 text-[10px] font-black uppercase tracking-[0.2em] text-zinc-400 dark:text-zinc-500 mb-3">
              <ServerIcon className="h-3.5 w-3.5" />
              Runtime Config
            </div>
            <div className="font-mono text-xs text-zinc-700 dark:text-zinc-300 break-all leading-relaxed bg-white/50 dark:bg-black/20 rounded-lg p-3 border border-zinc-200/50 dark:border-zinc-800/50">
              {server.type === 'sse' ? (
                server.url
              ) : (
                <>
                  <span className="font-bold text-indigo-600 dark:text-indigo-400">{server.command}</span>
                  {server.args && server.args.length > 0 && (
                    <span className="text-zinc-500"> {server.args.join(" ")}</span>
                  )}
                </>
              )}
            </div>
          </div>

          <div className="flex flex-wrap gap-2">
            {Object.entries(server.env || {}).slice(0, 3).map(([key, value]) => (
              <div key={key} className="flex items-center gap-2 rounded-xl border border-zinc-100 bg-white px-3 py-1.5 text-[10px] dark:border-zinc-800 dark:bg-zinc-950">
                <span className="font-black text-zinc-400 uppercase tracking-tighter">{key}</span>
                <span className="h-3 w-px bg-zinc-100 dark:bg-zinc-800" />
                <span className="font-mono font-medium text-zinc-600 dark:text-zinc-400 truncate max-w-[80px]">{value}</span>
              </div>
            ))}
            {Object.keys(server.env || {}).length > 3 && (
              <div className="rounded-xl bg-zinc-100 px-3 py-1.5 text-[10px] font-bold text-zinc-400 dark:bg-zinc-900">
                +{Object.keys(server.env).length - 3} more
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Action Bar */}
      <div className="flex items-center justify-between border-t border-zinc-50 bg-zinc-50/50 px-8 py-5 dark:border-zinc-900 dark:bg-zinc-900/20">
        <div className="flex gap-4">
          <div className="flex items-center gap-1.5 text-[11px] font-bold text-zinc-400">
            <Cpu className="h-3.5 w-3.5" />
            {server.type.toUpperCase()}
          </div>
          <div className="flex items-center gap-1.5 text-[11px] font-bold text-zinc-400">
            <Boxes className="h-3.5 w-3.5" />
            Namespaced
          </div>
        </div>
        
        <div className="flex items-center gap-2">
          <button
            onClick={() => onRestart(server.id)}
            className="flex h-9 w-9 items-center justify-center rounded-xl text-zinc-400 transition-all hover:bg-white hover:text-indigo-600 hover:shadow-md dark:hover:bg-zinc-800 dark:hover:text-indigo-400 active:rotate-180"
          >
            <RefreshCw className="h-4 w-4" />
          </button>
          <button
            onClick={() => onEdit(server)}
            className="flex h-9 w-9 items-center justify-center rounded-xl text-zinc-400 transition-all hover:bg-white hover:text-zinc-900 hover:shadow-md dark:hover:bg-zinc-800 dark:hover:text-zinc-200"
          >
            <Settings className="h-4 w-4" />
          </button>
          <button
            onClick={() => onDelete(server.id)}
            className="flex h-9 w-9 items-center justify-center rounded-xl text-zinc-400 transition-all hover:bg-red-50 hover:text-red-500 hover:shadow-md dark:hover:bg-zinc-800 dark:hover:text-red-400"
          >
            <Trash2 className="h-4 w-4" />
          </button>
        </div>
      </div>
    </motion.div>
  );
}
