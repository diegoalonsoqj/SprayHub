import type { Spray } from "@/domain/entities/spray";
import type { CreateSprayInput, WriterRepository } from "@/domain/repositories/writer-repository";

export class CreateSpray {
  constructor(private readonly repo: WriterRepository) {}

  execute(input: CreateSprayInput): Promise<Spray> {
    if (!input.libraryDir) {
      return Promise.reject(new Error("No library folder configured"));
    }
    if (!input.name) {
      return Promise.reject(new Error("Spray name is required"));
    }
    return this.repo.create(input);
  }
}
