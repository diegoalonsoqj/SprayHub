import type { AppConfig } from "@/domain/entities/config";
import type { ConfigRepository } from "@/domain/repositories/config-repository";

import { invoke } from "./invoke";

export class TauriConfigRepository implements ConfigRepository {
  load(): Promise<AppConfig> {
    return invoke<AppConfig>("get_config");
  }

  save(config: AppConfig): Promise<AppConfig> {
    return invoke<AppConfig>("save_config", { config });
  }
}
