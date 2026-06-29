/**
 * Composition root. Wires the Tauri infrastructure adapters into the
 * application use cases. The presentation layer depends on this single module
 * instead of constructing adapters ad hoc.
 */
import { ApplySprayUseCase } from "@/application/use-cases/apply-spray";
import { DetectGames } from "@/application/use-cases/detect-games";
import { ManageConfig } from "@/application/use-cases/manage-config";
import { ScanSprays } from "@/application/use-cases/scan-sprays";
import { TauriApplierRepository } from "@/infrastructure/tauri/tauri-applier-repository";
import { TauriConfigRepository } from "@/infrastructure/tauri/tauri-config-repository";
import { TauriGameRepository } from "@/infrastructure/tauri/tauri-game-repository";
import { TauriSprayRepository } from "@/infrastructure/tauri/tauri-spray-repository";

const sprayRepo = new TauriSprayRepository();
const gameRepo = new TauriGameRepository();
const configRepo = new TauriConfigRepository();
const applierRepo = new TauriApplierRepository();

export const container = {
  scanSprays: new ScanSprays(sprayRepo),
  detectGames: new DetectGames(gameRepo),
  manageConfig: new ManageConfig(configRepo),
  applySpray: new ApplySprayUseCase(applierRepo),
} as const;

export type Container = typeof container;
