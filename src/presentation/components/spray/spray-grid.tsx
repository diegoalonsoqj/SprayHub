import type { Spray } from "@/domain/entities/spray";

import { SprayCard } from "./spray-card";

interface SprayGridProps {
  sprays: Spray[];
  selectedId: string | null;
  favorites: string[];
  onSelect: (id: string) => void;
  onActivate: (spray: Spray) => void;
  onToggleFavorite: (id: string) => void;
  onDelete: (spray: Spray) => void;
}

export function SprayGrid({
  sprays,
  selectedId,
  favorites,
  onSelect,
  onActivate,
  onToggleFavorite,
  onDelete,
}: SprayGridProps) {
  return (
    <div className="grid grid-cols-[repeat(auto-fill,minmax(110px,1fr))] gap-3 p-4">
      {sprays.map((spray) => (
        <SprayCard
          key={spray.id}
          spray={spray}
          selected={spray.id === selectedId}
          favorite={favorites.includes(spray.id)}
          onSelect={onSelect}
          onActivate={onActivate}
          onToggleFavorite={onToggleFavorite}
          onDelete={onDelete}
        />
      ))}
    </div>
  );
}
