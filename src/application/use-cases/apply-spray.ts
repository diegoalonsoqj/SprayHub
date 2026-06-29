import type { ApplyResult, ApplySprayRequest } from "@/domain/entities/apply";
import type { ApplierRepository } from "@/domain/repositories/applier-repository";

export class ApplySprayUseCase {
  constructor(private readonly repo: ApplierRepository) {}

  execute(request: ApplySprayRequest): Promise<ApplyResult> {
    if (!request.vtfPath) {
      return Promise.reject(new Error("No spray selected"));
    }
    if (!request.destinationDir) {
      return Promise.reject(new Error("No destination configured"));
    }
    return this.repo.apply(request);
  }
}
