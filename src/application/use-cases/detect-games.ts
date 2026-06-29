import type { GameInfo } from "@/domain/entities/game";
import type { SteamDetection } from "@/domain/entities/steam";
import type { GameRepository } from "@/domain/repositories/game-repository";

export class DetectGames {
  constructor(private readonly repo: GameRepository) {}

  detectSteam(): Promise<SteamDetection> {
    return this.repo.detectSteam();
  }

  listGames(): Promise<GameInfo[]> {
    return this.repo.listGames();
  }
}
