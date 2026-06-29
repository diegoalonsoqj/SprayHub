import { useCallback, useEffect } from "react";

import type { AppConfig } from "@/domain/entities/config";
import { container } from "@/presentation/container";
import { setLanguage } from "@/presentation/i18n";
import { useAppStore } from "@/presentation/store/app-store";

import { toast } from "@/presentation/components/ui/toast";
import { useErrorMessage } from "./use-error-message";

/** Loads config on mount and exposes a persist function. */
export function useConfig() {
  const config = useAppStore((s) => s.config);
  const setConfig = useAppStore((s) => s.setConfig);
  const toMessage = useErrorMessage();

  useEffect(() => {
    let active = true;
    container.manageConfig
      .load()
      .then((loaded) => {
        if (!active) return;
        setConfig(loaded);
        setLanguage(loaded.language);
        applyTheme(loaded.theme);
      })
      .catch((err) => toast.error(toMessage(err)));
    return () => {
      active = false;
    };
  }, [setConfig, toMessage]);

  const save = useCallback(
    async (next: AppConfig): Promise<boolean> => {
      try {
        const saved = await container.manageConfig.save(next);
        setConfig(saved);
        setLanguage(saved.language);
        applyTheme(saved.theme);
        return true;
      } catch (err) {
        toast.error(toMessage(err));
        return false;
      }
    },
    [setConfig, toMessage],
  );

  return { config, save };
}

function applyTheme(theme: AppConfig["theme"]) {
  document.documentElement.classList.toggle("dark", theme === "dark");
}
