import type { GameInfo } from "@/domain/entities/game";
import type { SteamDetection } from "@/domain/entities/steam";

export interface GameRepository {
  detectSteam(): Promise<SteamDetection>;
  listGames(): Promise<GameInfo[]>;
}
