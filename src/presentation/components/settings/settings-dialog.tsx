import { open } from "@tauri-apps/plugin-dialog";
import { FolderOpen } from "lucide-react";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

import type { AppConfig, Language } from "@/domain/entities/config";
import type { GameInfo } from "@/domain/entities/game";
import type { SteamDetection } from "@/domain/entities/steam";
import { Button } from "@/presentation/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/presentation/components/ui/dialog";
import { Input } from "@/presentation/components/ui/input";
import { Label } from "@/presentation/components/ui/label";
import { Switch } from "@/presentation/components/ui/switch";

interface SettingsDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  config: AppConfig;
  games: GameInfo[];
  detection: SteamDetection | null;
  onSave: (config: AppConfig) => Promise<boolean>;
}

export function SettingsDialog({
  open: isOpen,
  onOpenChange,
  config,
  games,
  detection,
  onSave,
}: SettingsDialogProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AppConfig>(config);

  // Re-sync the draft whenever the dialog opens.
  useEffect(() => {
    if (isOpen) setDraft(config);
  }, [isOpen, config]);

  const patch = (partial: Partial<AppConfig>) => setDraft((d) => ({ ...d, ...partial }));

  const onPickGame = (gameId: string) => {
    const game = games.find((g) => g.id === gameId);
    patch({
      selectedGameId: gameId || null,
      // Auto-fill the destination from the chosen game.
      destinationDir: game?.spraysDir ?? draft.destinationDir,
    });
  };

  const browseLibrary = async () => {
    const selected = await open({ directory: true, multiple: false });
    if (typeof selected === "string") patch({ libraryDir: selected });
  };

  const submit = async () => {
    const ok = await onSave(draft);
    if (ok) onOpenChange(false);
  };

  return (
    <Dialog open={isOpen} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-xl">
        <DialogHeader>
          <DialogTitle>{t("settings.title")}</DialogTitle>
          <DialogDescription>{t("settings.description")}</DialogDescription>
        </DialogHeader>

        <div className="grid gap-4 py-2">
          {/* Library folder */}
          <div className="grid gap-1.5">
            <Label htmlFor="library">{t("settings.libraryDir")}</Label>
            <div className="flex gap-2">
              <Input
                id="library"
                value={draft.libraryDir ?? ""}
                onChange={(e) => patch({ libraryDir: e.target.value || null })}
                placeholder="C:\\Sprays"
              />
              <Button variant="outline" size="icon" onClick={browseLibrary}>
                <FolderOpen />
              </Button>
            </div>
          </div>

          {/* Game selector */}
          <div className="grid gap-1.5">
            <Label htmlFor="game">{t("settings.game")}</Label>
            <select
              id="game"
              value={draft.selectedGameId ?? ""}
              onChange={(e) => onPickGame(e.target.value)}
              className="flex h-9 w-full rounded-md border border-input bg-transparent px-3 text-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
            >
              <option value="">{t("settings.gamePlaceholder")}</option>
              {games.map((g) => (
                <option key={g.id} value={g.id} disabled={!g.installed}>
                  {g.name} {g.installed ? "" : t("settings.notInstalled")}
                </option>
              ))}
            </select>
          </div>

          {/* Destination */}
          <div className="grid gap-1.5">
            <Label htmlFor="destination">{t("settings.destinationDir")}</Label>
            <Input
              id="destination"
              value={draft.destinationDir ?? ""}
              onChange={(e) => patch({ destinationDir: e.target.value || null })}
            />
            <p className="text-xs text-muted-foreground">{t("settings.destinationHint")}</p>
          </div>

          {/* Behavior switches */}
          <div className="grid gap-3 rounded-md border border-border p-3">
            <SwitchRow
              id="backup"
              label={t("settings.createBackup")}
              checked={draft.createBackup}
              onChange={(v) => patch({ createBackup: v })}
            />
            <SwitchRow
              id="confirm"
              label={t("settings.confirmBeforeApply")}
              checked={draft.confirmBeforeApply}
              onChange={(v) => patch({ confirmBeforeApply: v })}
            />
            <SwitchRow
              id="dblclick"
              label={t("settings.applyOnDoubleClick")}
              checked={draft.applyOnDoubleClick}
              onChange={(v) => patch({ applyOnDoubleClick: v })}
            />
          </div>

          {/* Language */}
          <div className="grid gap-1.5">
            <Label htmlFor="language">{t("settings.language")}</Label>
            <select
              id="language"
              value={draft.language}
              onChange={(e) => patch({ language: e.target.value as Language })}
              className="flex h-9 w-full rounded-md border border-input bg-transparent px-3 text-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
            >
              <option value="es">Español</option>
              <option value="en">English</option>
            </select>
          </div>

          {/* Steam detection status */}
          <p className="text-xs text-muted-foreground">
            {detection?.steamRoot
              ? `${t("settings.steamDetected", { path: detection.steamRoot })} · ${t(
                  "settings.librariesFound",
                  { count: detection.libraries.length },
                )}`
              : t("settings.steamNotDetected")}
          </p>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            {t("settings.cancel")}
          </Button>
          <Button onClick={submit}>{t("settings.save")}</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

function SwitchRow({
  id,
  label,
  checked,
  onChange,
}: {
  id: string;
  label: string;
  checked: boolean;
  onChange: (value: boolean) => void;
}) {
  return (
    <div className="flex items-center justify-between gap-4">
      <Label htmlFor={id} className="cursor-pointer font-normal">
        {label}
      </Label>
      <Switch id={id} checked={checked} onCheckedChange={onChange} />
    </div>
  );
}
