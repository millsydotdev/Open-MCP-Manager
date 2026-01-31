import Link from "next/link";
import { Layers } from "lucide-react";

interface NavbarProps {
  onExport?: () => void;
}

export function Navbar({ onExport }: NavbarProps) {
  return (
    <nav className="sticky top-0 z-50 w-full border-b border-zinc-200 bg-white/80 backdrop-blur-md dark:border-zinc-800 dark:bg-black/80">
      <div className="mx-auto flex h-16 max-w-7xl items-center justify-between px-4 sm:px-6 lg:px-8">
        <div className="flex items-center gap-2">
          <Layers className="h-6 w-6 text-indigo-600 dark:text-indigo-400" />
          <span className="text-xl font-bold tracking-tight text-zinc-900 dark:text-white">
            MCP Manager
          </span>
        </div>
        <div className="flex items-center gap-4">
          <Link
            href="/"
            className="text-sm font-medium text-zinc-600 hover:text-indigo-600 dark:text-zinc-400 dark:hover:text-indigo-400"
          >
            Dashboard
          </Link>
          {onExport && (
            <button
              onClick={onExport}
              className="rounded-full bg-zinc-900 px-4 py-2 text-sm font-medium text-white transition-all hover:bg-zinc-800 dark:bg-white dark:text-black dark:hover:bg-zinc-200"
            >
              Export Config
            </button>
          )}
        </div>
      </div>
    </nav>
  );
}
