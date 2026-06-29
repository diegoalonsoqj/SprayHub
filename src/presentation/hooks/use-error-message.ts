import { useCallback } from "react";
import { useTranslation } from "react-i18next";

import { isCommandError } from "@/domain/entities/error";

/** Translate any thrown value (CommandError or Error) into a user message. */
export function useErrorMessage() {
  const { t } = useTranslation();
  return useCallback(
    (err: unknown): string => {
      if (isCommandError(err)) {
        const label = t(`errors.${err.category}`, { defaultValue: err.category });
        return `${label}: ${err.message}`;
      }
      if (err instanceof Error) return err.message;
      return String(err);
    },
    [t],
  );
}
