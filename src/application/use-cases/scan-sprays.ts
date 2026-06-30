import type { Spray } from "@/domain/entities/spray";
import type { SprayRepository } from "@/domain/repositories/spray-repository";

/** Scan a library and filter the results by a search query. */
export class ScanSprays {
  constructor(private readonly repo: SprayRepository) {}

  async execute(libraryDir: string): Promise<Spray[]> {
    return this.repo.scan(libraryDir);
  }

  thumbnail(vtfPath: string): Promise<string> {
    return this.repo.thumbnail(vtfPath);
  }

  delete(vtfPath: string, vmtPath: string | null): Promise<void> {
    return this.repo.delete(vtfPath, vmtPath);
  }

  /** Pure helper: instant client-side filtering by name. */
  static filter(sprays: Spray[], query: string): Spray[] {
    const q = query.trim().toLowerCase();
    if (!q) return sprays;
    return sprays.filter((s) => s.name.toLowerCase().includes(q));
  }
}
