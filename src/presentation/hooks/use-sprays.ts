import { useCallback, useEffect } from "react";

import { container } from "@/presentation/container";
import { useAppStore } from "@/presentation/store/app-store";

import { toast } from "@/presentation/components/ui/toast";
import { useErrorMessage } from "./use-error-message";

/** Scans the configured spray library; re-scans on demand. */
export function useSprays() {
  const libraryDir = useAppStore((s) => s.config.libraryDir);
  const sprays = useAppStore((s) => s.sprays);
  const setSprays = useAppStore((s) => s.setSprays);
  const loading = useAppStore((s) => s.loading);
  const setLoading = useAppStore((s) => s.setLoading);
  const toMessage = useErrorMessage();

  const refresh = useCallback(async () => {
    if (!libraryDir) {
      setSprays([]);
      return;
    }
    setLoading(true);
    try {
      const result = await container.scanSprays.execute(libraryDir);
      setSprays(result);
    } catch (err) {
      toast.error(toMessage(err));
      setSprays([]);
    } finally {
      setLoading(false);
    }
  }, [libraryDir, setSprays, setLoading, toMessage]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  return { sprays, loading, refresh };
}
