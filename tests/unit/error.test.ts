import { describe, expect, it } from "vitest";

import { isCommandError } from "@/domain/entities/error";

describe("isCommandError", () => {
  it("accepts the backend error shape", () => {
    expect(isCommandError({ category: "Steam", message: "boom" })).toBe(true);
  });

  it("rejects other values", () => {
    expect(isCommandError(null)).toBe(false);
    expect(isCommandError("nope")).toBe(false);
    expect(isCommandError({ category: "Steam" })).toBe(false);
  });
});
