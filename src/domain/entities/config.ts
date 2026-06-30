export type Language = "es" | "en";
export type Theme = "dark" | "light";
export type SprayFormat = "bgra8888" | "dxt5";

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
  sprayFormat: SprayFormat;
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
  sprayFormat: "bgra8888",
  favorites: [],
};
