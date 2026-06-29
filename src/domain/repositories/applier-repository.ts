import type { ApplyResult, ApplySprayRequest } from "@/domain/entities/apply";

export interface ApplierRepository {
  apply(request: ApplySprayRequest): Promise<ApplyResult>;
}
