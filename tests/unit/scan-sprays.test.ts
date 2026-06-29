import { describe, expect, it } from "vitest";

import { ScanSprays } from "@/application/use-cases/scan-sprays";
import type { Spray } from "@/domain/entities/spray";

function spray(name: string): Spray {
  return {
    id: name,
    name,
    vtfPath: `/sprays/${name}.vtf`,
    vmtPath: null,
    sizeBytes: 1024,
    modifiedAt: 0,
  };
}

describe("ScanSprays.filter", () => {
  const sprays = [spray("Dragon"), spray("dragonfly"), spray("Cat")];

  it("returns all sprays for an empty query", () => {
    expect(ScanSprays.filter(sprays, "")).toHaveLength(3);
    expect(ScanSprays.filter(sprays, "   ")).toHaveLength(3);
  });

  it("filters case-insensitively by name substring", () => {
    const result = ScanSprays.filter(sprays, "dragon");
    expect(result.map((s) => s.name)).toEqual(["Dragon", "dragonfly"]);
  });

  it("returns an empty array when nothing matches", () => {
    expect(ScanSprays.filter(sprays, "zzz")).toHaveLength(0);
  });
});
