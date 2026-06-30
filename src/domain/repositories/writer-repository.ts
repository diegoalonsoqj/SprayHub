import type { SprayFormat } from "@/domain/entities/config";
import type { Spray } from "@/domain/entities/spray";

export interface CreateSprayInput {
  name: string;
  width: number;
  height: number;
  /** Base64-encoded row-major RGBA8888 pixels. */
  rgbaBase64: string;
  format: SprayFormat;
  libraryDir: string;
}

export interface WriterRepository {
  create(input: CreateSprayInput): Promise<Spray>;
}
