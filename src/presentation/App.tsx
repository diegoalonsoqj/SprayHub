import { useEffect } from "react";

import { TooltipProvider } from "@/presentation/components/ui/tooltip";
import { Toaster } from "@/presentation/components/ui/toast";
import { LibraryPage } from "@/presentation/pages/library-page";

export function App() {
  // Suppress the webview's right-click context menu (this is a desktop app),
  // but keep it on editable fields so users can still cut/copy/paste.
  useEffect(() => {
    const onContextMenu = (e: MouseEvent) => {
      const target = e.target as HTMLElement | null;
      const editable = target?.closest("input, textarea, [contenteditable='true']");
      if (!editable) e.preventDefault();
    };
    document.addEventListener("contextmenu", onContextMenu);
    return () => document.removeEventListener("contextmenu", onContextMenu);
  }, []);

  return (
    <TooltipProvider delayDuration={300}>
      <LibraryPage />
      <Toaster />
    </TooltipProvider>
  );
}
