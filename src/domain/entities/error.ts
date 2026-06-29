export type ErrorCategory =
  "Config" | "Steam" | "Filesystem" | "Validation" | "NotFound" | "Internal";

/** Error payload returned by backend commands. */
export interface CommandError {
  category: ErrorCategory;
  message: string;
}

/** Type guard for the serialized backend error shape. */
export function isCommandError(value: unknown): value is CommandError {
  return typeof value === "object" && value !== null && "category" in value && "message" in value;
}
