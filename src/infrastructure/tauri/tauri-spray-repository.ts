import type { Spray } from "@/domain/entities/spray";
import type { SprayRepository } from "@/domain/repositories/spray-repository";

import { invoke } from "./invoke";

export class TauriSprayRepository implements SprayRepository {
  scan(libraryDir: string): Promise<Spray[]> {
    return invoke<Spray[]>("scan_sprays", { libraryDir });
  }

  thumbnail(vtfPath: string): Promise<string> {
    return invoke<string>("get_thumbnail", { vtfPath });
  }

  delete(vtfPath: string, vmtPath: string | null): Promise<void> {
    return invoke<void>("delete_spray", { vtfPath, vmtPath });
  }

  appliedNames(destinationDir: string): Promise<string[]> {
    return invoke<string[]>("applied_spray_names", { destinationDir });
  }
}
