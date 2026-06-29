import type { AppConfig } from "@/domain/entities/config";

export interface ConfigRepository {
  load(): Promise<AppConfig>;
  save(config: AppConfig): Promise<AppConfig>;
}
