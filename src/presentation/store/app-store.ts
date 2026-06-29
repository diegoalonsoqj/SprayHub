import { create } from "zustand";

import type { AppConfig } from "@/domain/entities/config";
import { defaultConfig } from "@/domain/entities/config";
import type { GameInfo } from "@/domain/entities/game";
import type { Spray } from "@/domain/entities/spray";

interface AppState {
  config: AppConfig;
  games: GameInfo[];
  sprays: Spray[];
  selectedSprayId: string | null;
  search: string;
  loading: boolean;

  setConfig: (config: AppConfig) => void;
  setGames: (games: GameInfo[]) => void;
  setSprays: (sprays: Spray[]) => void;
  selectSpray: (id: string | null) => void;
  setSearch: (search: string) => void;
  setLoading: (loading: boolean) => void;
  toggleFavorite: (id: string) => void;
}

export const useAppStore = create<AppState>((set) => ({
  config: defaultConfig,
  games: [],
  sprays: [],
  selectedSprayId: null,
  search: "",
  loading: false,

  setConfig: (config) => set({ config }),
  setGames: (games) => set({ games }),
  setSprays: (sprays) => set({ sprays }),
  selectSpray: (selectedSprayId) => set({ selectedSprayId }),
  setSearch: (search) => set({ search }),
  setLoading: (loading) => set({ loading }),
  toggleFavorite: (id) =>
    set((state) => {
      const favorites = state.config.favorites.includes(id)
        ? state.config.favorites.filter((f) => f !== id)
        : [...state.config.favorites, id];
      return { config: { ...state.config, favorites } };
    }),
}));
