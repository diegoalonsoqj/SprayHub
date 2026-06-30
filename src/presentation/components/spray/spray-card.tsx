import { CheckCircle2, ImageOff, Star, Trash2 } from "lucide-react";
import { memo } from "react";
import { useTranslation } from "react-i18next";

import type { Spray } from "@/domain/entities/spray";
import { cn, formatBytes } from "@/presentation/lib/utils";
import { useThumbnail } from "@/presentation/hooks/use-thumbnail";

interface SprayCardProps {
  spray: Spray;
  selected: boolean;
  favorite: boolean;
  applied: boolean;
  onSelect: (id: string) => void;
  onActivate: (spray: Spray) => void;
  onToggleFavorite: (id: string) => void;
  onDelete: (spray: Spray) => void;
}

function SprayCardImpl({
  spray,
  selected,
  favorite,
  applied,
  onSelect,
  onActivate,
  onToggleFavorite,
  onDelete,
}: SprayCardProps) {
  const { t } = useTranslation();
  const { ref, src, failed } = useThumbnail(spray.vtfPath);

  return (
    <button
      type="button"
      onClick={() => onSelect(spray.id)}
      onDoubleClick={() => onActivate(spray)}
      // Future feature: drag & drop. The draggable handle is wired now so the
      // backend apply flow can later accept dropped files too.
      draggable
      onDragStart={(e) => {
        e.dataTransfer.setData("application/x-sprayhub-id", spray.id);
        e.dataTransfer.effectAllowed = "copy";
      }}
      className={cn(
        "group relative flex flex-col overflow-hidden rounded-lg border bg-card text-left transition-all focus:outline-none focus-visible:ring-2 focus-visible:ring-ring",
        selected
          ? "border-primary ring-2 ring-primary/50"
          : "border-border hover:border-muted-foreground/40",
      )}
      title={spray.name}
    >
      <div
        ref={ref}
        className="relative flex aspect-square w-full items-center justify-center bg-[repeating-conic-gradient(theme(colors.border)_0%_25%,transparent_0%_50%)] bg-[length:16px_16px]"
      >
        {src ? (
          <img
            src={src}
            alt={spray.name}
            className="size-full object-contain p-2 [image-rendering:pixelated]"
            draggable={false}
          />
        ) : failed ? (
          <ImageOff className="size-6 text-muted-foreground" />
        ) : (
          <div className="size-6 animate-pulse rounded bg-muted" />
        )}

        {applied && (
          <span
            className="absolute bottom-1 right-1 rounded-full bg-background/80 p-0.5"
            title={t("library.inGame")}
          >
            <CheckCircle2 className="size-4 text-emerald-500" />
          </span>
        )}
      </div>

      <div className="flex items-center justify-between gap-1 px-2 py-1.5">
        <div className="min-w-0">
          <p className="truncate text-xs font-medium">{spray.name}</p>
          <p className="text-[10px] text-muted-foreground">{formatBytes(spray.sizeBytes)}</p>
        </div>
      </div>

      <span
        role="button"
        tabIndex={-1}
        onClick={(e) => {
          e.stopPropagation();
          onToggleFavorite(spray.id);
        }}
        className={cn(
          "absolute right-1.5 top-1.5 rounded-full bg-black/40 p-1 opacity-0 transition-opacity group-hover:opacity-100",
          favorite && "opacity-100",
        )}
        aria-label="Toggle favorite"
      >
        <Star
          className={cn("size-3.5", favorite ? "fill-yellow-400 text-yellow-400" : "text-white")}
        />
      </span>

      <span
        role="button"
        tabIndex={-1}
        onClick={(e) => {
          e.stopPropagation();
          onDelete(spray);
        }}
        className="absolute left-1.5 top-1.5 rounded-full bg-black/40 p-1 text-white opacity-0 transition-opacity hover:bg-destructive group-hover:opacity-100"
        aria-label="Delete spray"
      >
        <Trash2 className="size-3.5" />
      </span>
    </button>
  );
}

export const SprayCard = memo(SprayCardImpl);
