export type Language = "es" | "en";
export type Theme = "dark" | "light";

/** Persisted application configuration (mirrors the Rust `AppConfig`). */
export interface AppConfig {
  libraryDir: string | null;
  selectedGameId: string | null;
  destinationDir: string | null;
  createBackup: boolean;
  confirmBeforeApply: boolean;
  applyOnDoubleClick: boolean;
  language: Language;
  theme: Theme;
  favorites: string[];
}

export const defaultConfig: AppConfig = {
  libraryDir: null,
  selectedGameId: null,
  destinationDir: null,
  createBackup: true,
  confirmBeforeApply: true,
  applyOnDoubleClick: false,
  language: "es",
  theme: "dark",
  favorites: [],
};
