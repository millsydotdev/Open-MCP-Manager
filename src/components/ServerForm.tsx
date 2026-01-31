"use client";

import { useState, useEffect } from "react";
import { McpServer } from "@/lib/types";
import { X, Plus, Trash2, Globe, Terminal, Info, AlertCircle, Settings2 } from "lucide-react";
import { motion, AnimatePresence } from "framer-motion";

interface ServerFormProps {
  server?: McpServer | null;
  onSave: (server: Partial<McpServer>) => void;
  onCancel: () => void;
}

export function ServerForm({ server, onSave, onCancel }: ServerFormProps) {
  const [formData, setFormData] = useState<Partial<McpServer>>({
    name: "",
    type: "stdio",
    command: "",
    args: [],
    url: "",
    env: {},
    description: "",
    is_active: true,
  });

  const [argInput, setArgInput] = useState("");
  const [envKey, setEnvKey] = useState("");
  const [envValue, setEnvValue] = useState("");
  const [activeTab, setActiveTab] = useState<'general' | 'config' | 'env'>('general');

  useEffect(() => {
    if (server) {
      setFormData({
        ...server,
        type: server.type || "stdio",
        args: server.args || [],
        env: server.env || {},
      });
    }
  }, [server]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSave(formData);
  };

  const addArg = () => {
    if (argInput.trim()) {
      setFormData({ ...formData, args: [...(formData.args || []), argInput.trim()] });
      setArgInput("");
    }
  };

  const removeArg = (index: number) => {
    const newArgs = [...(formData.args || [])];
    newArgs.splice(index, 1);
    setFormData({ ...formData, args: newArgs });
  };

  const addEnv = () => {
    if (envKey.trim() && envValue.trim()) {
      setFormData({
        ...formData,
        env: { ...formData.env, [envKey.trim()]: envValue.trim() },
      });
      setEnvKey("");
      setEnvValue("");
    }
  };

  const removeEnv = (key: string) => {
    const newEnv = { ...formData.env };
    delete newEnv[key];
    setFormData({ ...formData, env: newEnv });
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-md">
      <motion.div
        initial={{ opacity: 0, scale: 0.9, y: 20 }}
        animate={{ opacity: 1, scale: 1, y: 0 }}
        className="w-full max-w-2xl overflow-hidden rounded-[32px] bg-white shadow-2xl dark:bg-zinc-950 border border-zinc-200 dark:border-zinc-800"
      >
        <div className="flex items-center justify-between border-b border-zinc-100 p-8 dark:border-zinc-900">
          <div className="flex items-center gap-3">
            <div className="flex h-12 w-12 items-center justify-center rounded-2xl bg-indigo-50 dark:bg-indigo-500/10">
              <Settings2 className="h-6 w-6 text-indigo-600 dark:text-indigo-400" />
            </div>
            <div>
              <h2 className="text-2xl font-bold text-zinc-900 dark:text-white">
                {server ? "Edit Server" : "Add New Server"}
              </h2>
              <p className="text-sm text-zinc-500">Configure your MCP server instance</p>
            </div>
          </div>
          <button onClick={onCancel} className="rounded-full p-2 hover:bg-zinc-100 dark:hover:bg-zinc-900 transition-colors">
            <X className="h-6 w-6 text-zinc-400" />
          </button>
        </div>

        <div className="flex border-b border-zinc-100 dark:border-zinc-900 px-8">
          {[
            { id: 'general', label: 'General', icon: Info },
            { id: 'config', label: 'Configuration', icon: Terminal },
            { id: 'env', label: 'Environment', icon: AlertCircle }
          ].map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id as any)}
              className={`flex items-center gap-2 border-b-2 px-4 py-4 text-sm font-semibold transition-all ${
                activeTab === tab.id 
                  ? 'border-indigo-600 text-indigo-600 dark:border-indigo-400 dark:text-indigo-400' 
                  : 'border-transparent text-zinc-400 hover:text-zinc-600 dark:hover:text-zinc-200'
              }`}
            >
              <tab.icon className="h-4 w-4" />
              {tab.label}
            </button>
          ))}
        </div>

        <form onSubmit={handleSubmit} className="p-8 space-y-6 max-h-[60vh] overflow-y-auto custom-scrollbar">
          <AnimatePresence mode="wait">
            {activeTab === 'general' && (
              <motion.div
                key="general"
                initial={{ opacity: 0, x: -10 }}
                animate={{ opacity: 1, x: 0 }}
                exit={{ opacity: 0, x: 10 }}
                className="space-y-6"
              >
                <div className="flex gap-4 p-1.5 bg-zinc-100 dark:bg-zinc-900 rounded-2xl">
                  <button
                    type="button"
                    onClick={() => setFormData({ ...formData, type: 'stdio' })}
                    className={`flex-1 flex items-center justify-center gap-2 py-3 text-sm font-bold rounded-xl transition-all ${formData.type === 'stdio' ? 'bg-white dark:bg-zinc-800 shadow-lg text-indigo-600 dark:text-indigo-400' : 'text-zinc-500 hover:text-zinc-700 dark:hover:text-zinc-300'}`}
                  >
                    <Terminal className="h-4 w-4" />
                    stdio (Local)
                  </button>
                  <button
                    type="button"
                    onClick={() => setFormData({ ...formData, type: 'sse' })}
                    className={`flex-1 flex items-center justify-center gap-2 py-3 text-sm font-bold rounded-xl transition-all ${formData.type === 'sse' ? 'bg-white dark:bg-zinc-800 shadow-lg text-indigo-600 dark:text-indigo-400' : 'text-zinc-500 hover:text-zinc-700 dark:hover:text-zinc-300'}`}
                  >
                    <Globe className="h-4 w-4" />
                    sse (Remote)
                  </button>
                </div>

                <div className="space-y-4">
                  <div>
                    <label className="block text-sm font-bold text-zinc-900 dark:text-zinc-100 mb-2">Name</label>
                    <input
                      type="text"
                      required
                      className="w-full rounded-2xl border border-zinc-200 bg-zinc-50 px-4 py-3 text-sm outline-none focus:border-indigo-500 focus:ring-4 focus:ring-indigo-500/10 dark:border-zinc-800 dark:bg-zinc-900 dark:text-white transition-all"
                      value={formData.name}
                      onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                      placeholder="e.g. github-mcp"
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-bold text-zinc-900 dark:text-zinc-100 mb-2">Description</label>
                    <textarea
                      rows={3}
                      className="w-full rounded-2xl border border-zinc-200 bg-zinc-50 px-4 py-3 text-sm outline-none focus:border-indigo-500 focus:ring-4 focus:ring-indigo-500/10 dark:border-zinc-800 dark:bg-zinc-900 dark:text-white transition-all resize-none"
                      value={formData.description || ""}
                      onChange={(e) => setFormData({ ...formData, description: e.target.value })}
                      placeholder="What does this server do?"
                    />
                  </div>
                </div>
              </motion.div>
            )}

            {activeTab === 'config' && (
              <motion.div
                key="config"
                initial={{ opacity: 0, x: -10 }}
                animate={{ opacity: 1, x: 0 }}
                exit={{ opacity: 0, x: 10 }}
                className="space-y-6"
              >
                {formData.type === 'stdio' ? (
                  <div className="space-y-6">
                    <div>
                      <label className="block text-sm font-bold text-zinc-900 dark:text-zinc-100 mb-2">Execution Command</label>
                      <input
                        type="text"
                        required={formData.type === 'stdio'}
                        className="w-full rounded-2xl border border-zinc-200 bg-zinc-50 px-4 py-3 text-sm font-mono focus:border-indigo-500 focus:ring-4 focus:ring-indigo-500/10 dark:border-zinc-800 dark:bg-zinc-900 dark:text-white transition-all"
                        value={formData.command || ""}
                        onChange={(e) => setFormData({ ...formData, command: e.target.value })}
                        placeholder="e.g. npx"
                      />
                    </div>
                    <div>
                      <label className="block text-sm font-bold text-zinc-900 dark:text-zinc-100 mb-2">Arguments</label>
                      <div className="flex gap-2">
                        <input
                          type="text"
                          className="flex-1 rounded-2xl border border-zinc-200 bg-zinc-50 px-4 py-3 text-sm focus:border-indigo-500 focus:ring-4 focus:ring-indigo-500/10 dark:border-zinc-800 dark:bg-zinc-900 dark:text-white transition-all"
                          value={argInput}
                          onChange={(e) => setArgInput(e.target.value)}
                          onKeyPress={(e) => e.key === "Enter" && (e.preventDefault(), addArg())}
                          placeholder="Add CLI argument..."
                        />
                        <button
                          type="button"
                          onClick={addArg}
                          className="rounded-2xl bg-zinc-100 p-3 text-zinc-600 hover:bg-zinc-200 dark:bg-zinc-900 dark:text-zinc-400 dark:hover:bg-zinc-800"
                        >
                          <Plus className="h-5 w-5" />
                        </button>
                      </div>
                      <div className="mt-4 flex flex-wrap gap-2">
                        {formData.args?.map((arg, i) => (
                          <span key={i} className="inline-flex items-center gap-2 rounded-xl bg-indigo-50 px-3 py-1.5 text-xs font-semibold text-indigo-700 dark:bg-indigo-500/10 dark:text-indigo-400">
                            {arg}
                            <button type="button" onClick={() => removeArg(i)} className="hover:text-indigo-900 dark:hover:text-white">
                              <X className="h-3.5 w-3.5" />
                            </button>
                          </span>
                        ))}
                      </div>
                    </div>
                  </div>
                ) : (
                  <div>
                    <label className="block text-sm font-bold text-zinc-900 dark:text-zinc-100 mb-2">SSE Endpoint URL</label>
                    <input
                      type="url"
                      required={formData.type === 'sse'}
                      className="w-full rounded-2xl border border-zinc-200 bg-zinc-50 px-4 py-3 text-sm font-mono focus:border-indigo-500 focus:ring-4 focus:ring-indigo-500/10 dark:border-zinc-800 dark:bg-zinc-900 dark:text-white transition-all"
                      value={formData.url || ""}
                      onChange={(e) => setFormData({ ...formData, url: e.target.value })}
                      placeholder="https://example.com/mcp"
                    />
                    <p className="mt-2 text-xs text-zinc-500">The server must support SSE transport according to the MCP spec.</p>
                  </div>
                )}
              </motion.div>
            )}

            {activeTab === 'env' && (
              <motion.div
                key="env"
                initial={{ opacity: 0, x: -10 }}
                animate={{ opacity: 1, x: 0 }}
                exit={{ opacity: 0, x: 10 }}
                className="space-y-6"
              >
                <div>
                  <label className="block text-sm font-bold text-zinc-900 dark:text-zinc-100 mb-2">Environment Variables</label>
                  <div className="flex gap-2">
                    <input
                      type="text"
                      className="w-1/3 rounded-2xl border border-zinc-200 bg-zinc-50 px-4 py-3 text-sm font-mono focus:border-indigo-500 focus:ring-4 focus:ring-indigo-500/10 dark:border-zinc-800 dark:bg-zinc-900 dark:text-white"
                      value={envKey}
                      onChange={(e) => setEnvKey(e.target.value)}
                      placeholder="KEY"
                    />
                    <input
                      type="text"
                      className="flex-1 rounded-2xl border border-zinc-200 bg-zinc-50 px-4 py-3 text-sm font-mono focus:border-indigo-500 focus:ring-4 focus:ring-indigo-500/10 dark:border-zinc-800 dark:bg-zinc-900 dark:text-white"
                      value={envValue}
                      onChange={(e) => setEnvValue(e.target.value)}
                      placeholder="VALUE"
                    />
                    <button
                      type="button"
                      onClick={addEnv}
                      className="rounded-2xl bg-zinc-100 p-3 text-zinc-600 hover:bg-zinc-200 dark:bg-zinc-900 dark:text-zinc-400"
                    >
                      <Plus className="h-5 w-5" />
                    </button>
                  </div>
                </div>
                
                <div className="grid gap-3">
                  {Object.entries(formData.env || {}).map(([key, value]) => (
                    <div key={key} className="flex items-center justify-between rounded-2xl bg-zinc-50 p-4 dark:bg-zinc-900 border border-zinc-100 dark:border-zinc-800">
                      <div className="flex flex-col">
                        <span className="text-[10px] font-bold uppercase text-zinc-400 tracking-wider">Key</span>
                        <span className="font-mono text-sm font-bold text-indigo-600 dark:text-indigo-400">{key}</span>
                      </div>
                      <div className="flex flex-col flex-1 px-4">
                        <span className="text-[10px] font-bold uppercase text-zinc-400 tracking-wider">Value</span>
                        <span className="font-mono text-sm truncate max-w-[150px] dark:text-zinc-300">{value}</span>
                      </div>
                      <button type="button" onClick={() => removeEnv(key)} className="rounded-xl p-2 text-zinc-400 hover:bg-red-50 hover:text-red-500 transition-colors">
                        <Trash2 className="h-5 w-5" />
                      </button>
                    </div>
                  ))}
                </div>
              </motion.div>
            )}
          </AnimatePresence>
        </form>

        <div className="border-t border-zinc-100 p-8 bg-zinc-50 dark:bg-zinc-950 dark:border-zinc-900 flex justify-end gap-4 rounded-b-[32px]">
          <button
            type="button"
            onClick={onCancel}
            className="rounded-2xl px-6 py-3 text-sm font-bold text-zinc-500 hover:bg-zinc-200 dark:hover:bg-zinc-900 transition-all"
          >
            Cancel
          </button>
          <button
            type="submit"
            onClick={handleSubmit}
            className="rounded-2xl bg-indigo-600 px-8 py-3 text-sm font-bold text-white shadow-xl shadow-indigo-500/20 hover:bg-indigo-700 active:scale-95 transition-all"
          >
            {server ? "Save Changes" : "Create Server"}
          </button>
        </div>
      </motion.div>
    </div>
  );
}
