import type { Spray } from "@/domain/entities/spray";

export interface SprayRepository {
  scan(libraryDir: string): Promise<Spray[]>;
  thumbnail(vtfPath: string): Promise<string>;
  delete(vtfPath: string, vmtPath: string | null): Promise<void>;
  /** Base names of `.vtf` files already present in a game's destination folder. */
  appliedNames(destinationDir: string): Promise<string[]>;
}
