import { useCallback, useEffect, useState } from "react";

import type { SteamDetection } from "@/domain/entities/steam";
import { container } from "@/presentation/container";
import { useAppStore } from "@/presentation/store/app-store";

import { toast } from "@/presentation/components/ui/toast";
import { useErrorMessage } from "./use-error-message";

/** Detects Steam and lists supported games. */
export function useGames() {
  const games = useAppStore((s) => s.games);
  const setGames = useAppStore((s) => s.setGames);
  const [detection, setDetection] = useState<SteamDetection | null>(null);
  const toMessage = useErrorMessage();

  const refresh = useCallback(async () => {
    try {
      const [det, list] = await Promise.all([
        container.detectGames.detectSteam(),
        container.detectGames.listGames(),
      ]);
      setDetection(det);
      setGames(list);
    } catch (err) {
      toast.error(toMessage(err));
    }
  }, [setGames, toMessage]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  return { games, detection, refresh };
}
