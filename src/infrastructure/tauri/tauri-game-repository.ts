import type { GameInfo } from "@/domain/entities/game";
import type { SteamDetection } from "@/domain/entities/steam";
import type { GameRepository } from "@/domain/repositories/game-repository";

import { invoke } from "./invoke";

export class TauriGameRepository implements GameRepository {
  detectSteam(): Promise<SteamDetection> {
    return invoke<SteamDetection>("detect_steam");
  }

  listGames(): Promise<GameInfo[]> {
    return invoke<GameInfo[]>("list_games");
  }
}
