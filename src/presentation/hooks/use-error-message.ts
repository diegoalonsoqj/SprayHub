import { useCallback, useRef } from "react";
import { useTranslation } from "react-i18next";

import { isCommandError } from "@/domain/entities/error";

/**
 * Translate any thrown value (CommandError or Error) into a user message.
 *
 * The returned function has a STABLE identity (empty dependency array): it reads
 * the latest `t` through a ref. This matters because the function is used inside
 * other hooks' `useCallback`/`useEffect` dependency arrays — react-i18next can
 * hand back a new `t` reference on each render, and depending on it directly
 * would re-create those callbacks every render and cause effects (e.g. the
 * library scan) to loop endlessly.
 */
export function useErrorMessage() {
  const { t } = useTranslation();
  const tRef = useRef(t);
  tRef.current = t;

  return useCallback((err: unknown): string => {
    const translate = tRef.current;
    if (isCommandError(err)) {
      const label = translate(`errors.${err.category}`, { defaultValue: err.category });
      return `${label}: ${err.message}`;
    }
    if (err instanceof Error) return err.message;
    return String(err);
  }, []);
}
