/** Result of detecting Steam on the system. */
export interface SteamDetection {
  steamRoot: string | null;
  libraries: string[];
}
