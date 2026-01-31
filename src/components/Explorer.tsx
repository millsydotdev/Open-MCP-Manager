"use client";

import { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Search, Globe, Download, X, Loader2, Star, ExternalLink, ChevronDown, ChevronUp, Package, ShieldCheck, Zap } from "lucide-react";
import { searchRegistry } from "@/app/actions";
import { McpServer } from "@/lib/types";
import { toast } from "sonner";

interface ExplorerProps {
  onInstall: (server: Partial<McpServer>) => void;
  onClose: () => void;
}

export function Explorer({ onInstall, onClose }: ExplorerProps) {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [expandedIdx, setExpandedIdx] = useState<number | null>(null);

  const handleSearch = async (e?: React.FormEvent) => {
    e?.preventDefault();
    setLoading(true);
    try {
      const servers = await searchRegistry(query);
      setResults(servers);
    } catch (error) {
      toast.error("Failed to fetch registry");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    const timer = setTimeout(() => {
      handleSearch();
    }, 500);
    return () => clearTimeout(timer);
  }, [query]);

  const installServer = (regServer: any) => {
    const pkg = regServer.server.packages?.[0];
    if (!pkg) {
      toast.error("No installation package found for this server");
      return;
    }

    let command = pkg.runtimeHint || (pkg.registryType === 'npm' ? 'npx' : pkg.identifier);
    let args: string[] = [];
    
    if (pkg.registryType === 'npm') {
      command = 'npx';
      args = ['-y', pkg.identifier, ...(pkg.packageArguments?.map((arg: any) => arg.default || "") || [])];
    } else {
      args = pkg.packageArguments?.map((arg: any) => arg.default || "") || [];
    }

    const serverData: Partial<McpServer> = {
      name: regServer.server.title || regServer.server.name.split('/').pop() || regServer.server.name,
      type: pkg.transport?.type === 'sse' ? 'sse' : 'stdio',
      description: regServer.server.description,
      command: command,
      args: args,
      url: pkg.transport?.url || "",
      env: regServer.server.environmentVariables?.reduce((acc: any, curr: any) => ({ ...acc, [curr.key]: curr.value || "" }), {}) || {},
    };

    onInstall(serverData);
    toast.success(`Prepared ${serverData.name} for configuration`);
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-md">
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="relative flex h-[90vh] w-full max-w-6xl flex-col overflow-hidden rounded-[40px] bg-white shadow-2xl dark:bg-zinc-950 dark:border dark:border-zinc-800"
      >
        <div className="flex items-center justify-between border-b border-zinc-100 p-10 dark:border-zinc-900">
          <div>
            <div className="flex items-center gap-3 mb-2">
              <div className="flex h-10 w-10 items-center justify-center rounded-xl bg-indigo-600 text-white">
                <Globe className="h-6 w-6" />
              </div>
              <h2 className="text-3xl font-black tracking-tight text-zinc-900 dark:text-white">
                MCP Registry
              </h2>
            </div>
            <p className="text-lg font-medium text-zinc-500 dark:text-zinc-400">
              Discover, explore, and install Model Context Protocol servers.
            </p>
          </div>
          <button
            onClick={onClose}
            className="rounded-full p-3 hover:bg-zinc-100 dark:hover:bg-zinc-900 transition-colors"
          >
            <X className="h-8 w-8 text-zinc-400" />
          </button>
        </div>

        <div className="px-10 py-6 border-b border-zinc-50 dark:border-zinc-900/50">
          <form onSubmit={handleSearch} className="relative">
            <Search className="absolute left-6 top-1/2 h-6 w-6 -translate-y-1/2 text-zinc-400" />
            <input
              type="text"
              placeholder="Search by name, category, or functionality..."
              className="w-full rounded-[24px] border-2 border-zinc-100 bg-zinc-50 py-5 pl-16 pr-6 text-lg font-bold text-zinc-900 outline-none transition-all focus:border-indigo-500 focus:bg-white focus:ring-8 focus:ring-indigo-500/5 dark:border-zinc-800 dark:bg-zinc-900 dark:text-white dark:focus:border-indigo-500"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
            />
          </form>
        </div>

        <div className="flex-1 overflow-y-auto p-10 custom-scrollbar">
          {loading ? (
            <div className="flex h-full flex-col items-center justify-center space-y-6">
              <Loader2 className="h-16 w-16 animate-spin text-indigo-500" />
              <p className="text-xl font-black text-zinc-400 uppercase tracking-widest">Querying Global Registry...</p>
            </div>
          ) : results.length > 0 ? (
            <div className="grid gap-8">
              {results.map((item, idx) => (
                <motion.div
                  key={idx}
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  className={`relative flex flex-col rounded-[32px] border-2 transition-all ${
                    expandedIdx === idx 
                      ? 'border-indigo-500 bg-indigo-50/10 dark:bg-indigo-500/5' 
                      : 'border-zinc-100 bg-white hover:border-zinc-200 dark:border-zinc-800 dark:bg-zinc-900/50'
                  }`}
                >
                  <div className="p-8">
                    <div className="flex items-start justify-between">
                      <div className="flex-1">
                        <div className="flex items-center gap-3 mb-2">
                          <h3 className="text-2xl font-black tracking-tight text-zinc-900 dark:text-white">
                            {item.server.title || item.server.name.split('/').pop()}
                          </h3>
                          <div className="flex items-center gap-1 rounded-full bg-indigo-50 px-3 py-1 text-[10px] font-black uppercase tracking-widest text-indigo-600 dark:bg-indigo-500/10 dark:text-indigo-400">
                            v{item.server.version}
                          </div>
                          {item._meta?.isVerified && (
                            <ShieldCheck className="h-5 w-5 text-green-500" />
                          )}
                        </div>
                        <p className="text-xs font-mono text-zinc-400 mb-4">{item.server.name}</p>
                        <p className="text-lg text-zinc-600 dark:text-zinc-400 leading-relaxed max-w-3xl">
                          {item.server.description}
                        </p>
                      </div>
                      <div className="flex flex-col items-end gap-3">
                        {item.server.repository?.url && (
                          <a 
                            href={item.server.repository.url} 
                            target="_blank" 
                            rel="noopener noreferrer"
                            className="flex items-center gap-2 rounded-xl bg-zinc-100 px-4 py-2 text-xs font-bold text-zinc-600 hover:bg-zinc-200 dark:bg-zinc-800 dark:text-zinc-400"
                          >
                            GitHub <ExternalLink className="h-3 w-3" />
                          </a>
                        )}
                        <button
                          onClick={() => installServer(item)}
                          className="flex items-center gap-2 rounded-2xl bg-indigo-600 px-8 py-3.5 text-sm font-black text-white shadow-xl shadow-indigo-500/30 transition-all hover:-translate-y-1 hover:bg-indigo-700 active:scale-95"
                        >
                          <Download className="h-5 w-5" />
                          Install
                        </button>
                      </div>
                    </div>

                    <div className="mt-8 flex items-center justify-between">
                      <div className="flex gap-4">
                        {item.server.packages?.map((pkg: any, i: number) => (
                          <div key={i} className="flex items-center gap-2 rounded-xl bg-zinc-50 px-4 py-2 text-[11px] font-bold text-zinc-500 dark:bg-zinc-800/50">
                            <Package className="h-4 w-4" />
                            {pkg.registryType.toUpperCase()}
                            <span className="opacity-20">|</span>
                            {pkg.transport?.type.toUpperCase()}
                          </div>
                        ))}
                      </div>
                      <button 
                        onClick={() => setExpandedIdx(expandedIdx === idx ? null : idx)}
                        className="flex items-center gap-2 text-sm font-black text-indigo-600 dark:text-indigo-400"
                      >
                        {expandedIdx === idx ? 'Less Info' : 'More Information'}
                        {expandedIdx === idx ? <ChevronUp className="h-4 w-4" /> : <ChevronDown className="h-4 w-4" />}
                      </button>
                    </div>
                  </div>

                  <AnimatePresence>
                    {expandedIdx === idx && (
                      <motion.div
                        initial={{ height: 0, opacity: 0 }}
                        animate={{ height: 'auto', opacity: 1 }}
                        exit={{ height: 0, opacity: 0 }}
                        className="overflow-hidden border-t border-zinc-100 dark:border-zinc-800"
                      >
                        <div className="p-10 grid grid-cols-2 gap-10 bg-zinc-50/50 dark:bg-black/20">
                          <div>
                            <h4 className="text-xs font-black uppercase tracking-widest text-zinc-400 mb-4">Required Environment Variables</h4>
                            {item.server.environmentVariables?.length > 0 ? (
                              <div className="space-y-3">
                                {item.server.environmentVariables.map((env: any, i: number) => (
                                  <div key={i} className="rounded-xl border border-zinc-200 bg-white p-4 dark:border-zinc-800 dark:bg-zinc-900">
                                    <div className="flex items-center justify-between mb-1">
                                      <code className="font-mono text-sm font-bold text-indigo-600 dark:text-indigo-400">{env.key}</code>
                                      {env.isRequired && <span className="text-[10px] font-black text-red-500 uppercase">Required</span>}
                                    </div>
                                    <p className="text-xs text-zinc-500">{env.description || 'No description provided.'}</p>
                                  </div>
                                ))}
                              </div>
                            ) : (
                              <p className="text-sm font-medium text-zinc-500 italic">No environment variables required.</p>
                            )}
                          </div>
                          <div>
                            <h4 className="text-xs font-black uppercase tracking-widest text-zinc-400 mb-4">Installation Context</h4>
                            <div className="space-y-6">
                              <div className="rounded-2xl bg-zinc-900 p-6 text-zinc-300">
                                <p className="text-[10px] font-black uppercase tracking-widest text-zinc-500 mb-3">Runtime Hint</p>
                                <code className="font-mono text-sm block bg-black/40 p-4 rounded-xl border border-zinc-800">
                                  {item.server.packages?.[0]?.identifier}@{item.server.version}
                                </code>
                              </div>
                              <div className="flex gap-4">
                                <div className="flex-1 rounded-2xl border border-zinc-200 p-6 dark:border-zinc-800">
                                  <p className="text-[10px] font-black uppercase tracking-widest text-zinc-400 mb-2">Capabilities</p>
                                  <div className="flex flex-wrap gap-2">
                                    <span className="rounded-full bg-green-50 px-3 py-1 text-[10px] font-bold text-green-600 dark:bg-green-500/10">Tools</span>
                                    <span className="rounded-full bg-blue-50 px-3 py-1 text-[10px] font-bold text-blue-600 dark:bg-blue-500/10">Resources</span>
                                    <span className="rounded-full bg-purple-50 px-3 py-1 text-[10px] font-bold text-purple-600 dark:bg-purple-500/10">Prompts</span>
                                  </div>
                                </div>
                                <div className="flex-1 rounded-2xl border border-zinc-200 p-6 dark:border-zinc-800">
                                  <p className="text-[10px] font-black uppercase tracking-widest text-zinc-400 mb-2">Last Updated</p>
                                  <p className="text-lg font-black text-zinc-900 dark:text-white">
                                    {new Date().toLocaleDateString()}
                                  </p>
                                </div>
                              </div>
                            </div>
                          </div>
                        </div>
                      </motion.div>
                    )}
                  </AnimatePresence>
                </motion.div>
              ))}
            </div>
          ) : query ? (
            <div className="flex h-full flex-col items-center justify-center text-zinc-500">
              <Globe className="mb-6 h-20 w-20 opacity-10" />
              <h3 className="text-2xl font-black text-zinc-900 dark:text-white">No results found</h3>
              <p className="text-lg font-medium text-zinc-500">Try broadening your search or checking for typos.</p>
            </div>
          ) : (
            <div className="flex h-full flex-col items-center justify-center text-zinc-400">
              <div className="relative mb-8">
                <div className="absolute inset-0 animate-pulse bg-indigo-500/20 blur-[100px] rounded-full" />
                <Globe className="h-32 w-32 opacity-10" />
              </div>
              <h3 className="text-2xl font-black text-zinc-900 dark:text-white">Global MCP Registry</h3>
              <p className="text-lg font-medium text-zinc-500 mt-2">Search over 500+ official and community servers</p>
              <div className="mt-10 flex gap-4">
                {['Database', 'Cloud', 'AI', 'Search'].map((tag) => (
                  <button 
                    key={tag}
                    onClick={() => setQuery(tag)}
                    className="px-6 py-2.5 rounded-full border border-zinc-200 text-sm font-bold text-zinc-600 hover:border-indigo-500 hover:text-indigo-600 transition-all dark:border-zinc-800 dark:text-zinc-400"
                  >
                    {tag}
                  </button>
                ))}
              </div>
            </div>
          )}
        </div>
      </motion.div>
    </div>
  );
}
