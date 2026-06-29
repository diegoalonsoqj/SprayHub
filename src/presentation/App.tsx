import { TooltipProvider } from "@/presentation/components/ui/tooltip";
import { Toaster } from "@/presentation/components/ui/toast";
import { LibraryPage } from "@/presentation/pages/library-page";

export function App() {
  return (
    <TooltipProvider delayDuration={300}>
      <LibraryPage />
      <Toaster />
    </TooltipProvider>
  );
}
