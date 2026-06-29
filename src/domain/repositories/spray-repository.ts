import type { Spray } from "@/domain/entities/spray";

export interface SprayRepository {
  scan(libraryDir: string): Promise<Spray[]>;
  thumbnail(vtfPath: string): Promise<string>;
}
