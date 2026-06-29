import { invoke as tauriInvoke } from "@tauri-apps/api/core";

import { isCommandError, type CommandError } from "@/domain/entities/error";

/**
 * Thin wrapper around Tauri's `invoke` that normalizes backend errors into a
 * consistent `CommandError`. The infrastructure layer is the only place that
 * talks to the Tauri bridge.
 */
export async function invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await tauriInvoke<T>(command, args);
  } catch (err) {
    if (isCommandError(err)) {
      throw err;
    }
    const normalized: CommandError = {
      category: "Internal",
      message: typeof err === "string" ? err : String(err),
    };
    throw normalized;
  }
}
