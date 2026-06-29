import type { ApplyResult, ApplySprayRequest } from "@/domain/entities/apply";
import type { ApplierRepository } from "@/domain/repositories/applier-repository";

import { invoke } from "./invoke";

export class TauriApplierRepository implements ApplierRepository {
  apply(request: ApplySprayRequest): Promise<ApplyResult> {
    return invoke<ApplyResult>("apply_spray", { request });
  }
}
