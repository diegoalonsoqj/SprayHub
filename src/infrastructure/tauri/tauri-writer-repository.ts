import type { Spray } from "@/domain/entities/spray";
import type { CreateSprayInput, WriterRepository } from "@/domain/repositories/writer-repository";

import { invoke } from "./invoke";

export class TauriWriterRepository implements WriterRepository {
  create(input: CreateSprayInput): Promise<Spray> {
    return invoke<Spray>("create_spray", {
      name: input.name,
      width: input.width,
      height: input.height,
      rgbaBase64: input.rgbaBase64,
      format: input.format,
      libraryDir: input.libraryDir,
    });
  }
}
