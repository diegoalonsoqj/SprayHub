import type { AppConfig } from "@/domain/entities/config";
import type { ConfigRepository } from "@/domain/repositories/config-repository";

export class ManageConfig {
  constructor(private readonly repo: ConfigRepository) {}

  load(): Promise<AppConfig> {
    return this.repo.load();
  }

  save(config: AppConfig): Promise<AppConfig> {
    return this.repo.save(config);
  }
}
