import { useEffect, useRef, useState } from "react";

import { container } from "@/presentation/container";

/**
 * Lazily decode a VTF thumbnail when its host element scrolls into view, using
 * an IntersectionObserver. Keeps the grid responsive for large libraries.
 */
export function useThumbnail(vtfPath: string) {
  const ref = useRef<HTMLDivElement | null>(null);
  const [src, setSrc] = useState<string | null>(null);
  const [failed, setFailed] = useState(false);

  useEffect(() => {
    const node = ref.current;
    if (!node) return;

    let cancelled = false;
    const observer = new IntersectionObserver(
      (entries) => {
        const entry = entries[0];
        if (entry?.isIntersecting) {
          observer.disconnect();
          container.scanSprays
            .thumbnail(vtfPath)
            .then((dataUrl) => {
              if (!cancelled) setSrc(dataUrl);
            })
            .catch(() => {
              if (!cancelled) setFailed(true);
            });
        }
      },
      { rootMargin: "120px" },
    );

    observer.observe(node);
    return () => {
      cancelled = true;
      observer.disconnect();
    };
  }, [vtfPath]);

  return { ref, src, failed };
}
