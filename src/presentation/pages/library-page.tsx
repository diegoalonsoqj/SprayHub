import { FolderInput, ImageOff, SearchX } from "lucide-react";
import { useCallback, useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";

import type { Spray } from "@/domain/entities/spray";
import type { ApplySprayRequest } from "@/domain/entities/apply";
import { ScanSprays } from "@/application/use-cases/scan-sprays";
import { container } from "@/presentation/container";
import { fileToSprayImage } from "@/presentation/lib/image";
import { useAppStore } from "@/presentation/store/app-store";
import { useConfig } from "@/presentation/hooks/use-config";
import { useGames } from "@/presentation/hooks/use-games";
import { useSprays } from "@/presentation/hooks/use-sprays";
import { useErrorMessage } from "@/presentation/hooks/use-error-message";

import { ApplyConfirmDialog } from "@/presentation/components/apply/apply-confirm-dialog";
import { DeleteConfirmDialog } from "@/presentation/components/spray/delete-confirm-dialog";
import { EmptyState } from "@/presentation/components/common/empty-state";
import { Button } from "@/presentation/components/ui/button";
import { toast } from "@/presentation/components/ui/toast";
import { Toolbar } from "@/presentation/components/layout/toolbar";
import { SettingsDialog } from "@/presentation/components/settings/settings-dialog";
import { SprayGrid } from "@/presentation/components/spray/spray-grid";

export function LibraryPage() {
  const { t } = useTranslation();
  const { config, save } = useConfig();
  const { games, detection } = useGames();
  const { sprays, loading, refresh } = useSprays();
  const toMessage = useErrorMessage();

  const search = useAppStore((s) => s.search);
  const setSearch = useAppStore((s) => s.setSearch);
  const selectedId = useAppStore((s) => s.selectedSprayId);
  const selectSpray = useAppStore((s) => s.selectSpray);
  const toggleFavorite = useAppStore((s) => s.toggleFavorite);

  const [settingsOpen, setSettingsOpen] = useState(false);
  const [pendingSpray, setPendingSpray] = useState<Spray | null>(null);
  const [pendingDelete, setPendingDelete] = useState<Spray | null>(null);
  const [dragging, setDragging] = useState(false);
  const [appliedNames, setAppliedNames] = useState<Set<string>>(new Set());

  const filtered = useMemo(() => ScanSprays.filter(sprays, search), [sprays, search]);

  const selectedGame = games.find((g) => g.id === config.selectedGameId) ?? null;
  const destinationDir = config.destinationDir ?? selectedGame?.spraysDir ?? null;
  const selectedSpray = sprays.find((s) => s.id === selectedId) ?? null;
  const canApply = Boolean(selectedSpray && destinationDir);

  // Flag sprays whose files already exist in the game's destination folder.
  const refreshApplied = useCallback(async () => {
    if (!destinationDir) {
      setAppliedNames(new Set());
      return;
    }
    try {
      const names = await container.scanSprays.appliedNames(destinationDir);
      setAppliedNames(new Set(names.map((n) => n.toLowerCase())));
    } catch {
      setAppliedNames(new Set());
    }
  }, [destinationDir]);

  useEffect(() => {
    void refreshApplied();
  }, [refreshApplied]);

  const doApply = async (spray: Spray) => {
    if (!destinationDir) {
      toast.error(t("apply.noDestination"));
      return;
    }
    const request: ApplySprayRequest = {
      sprayId: spray.id,
      vtfPath: spray.vtfPath,
      vmtPath: spray.vmtPath,
      destinationDir,
      createBackup: config.createBackup,
      overwrite: true,
    };
    try {
      await container.applySpray.execute(request);
      toast.success(t("apply.success", { name: spray.name }));
      await refreshApplied();
    } catch (err) {
      toast.error(toMessage(err));
    }
  };

  const requestApply = (spray: Spray | null) => {
    if (!spray) {
      toast.info(t("apply.noSelection"));
      return;
    }
    if (!destinationDir) {
      toast.error(t("apply.noDestination"));
      return;
    }
    if (config.confirmBeforeApply) {
      setPendingSpray(spray);
    } else {
      void doApply(spray);
    }
  };

  const onActivate = (spray: Spray) => {
    selectSpray(spray.id);
    if (config.applyOnDoubleClick) requestApply(spray);
  };

  const handleImportFile = async (file: File) => {
    if (!config.libraryDir) {
      toast.error(t("import.noLibrary"));
      return;
    }
    try {
      const image = await fileToSprayImage(file);
      const created = await container.createSpray.execute({
        name: image.name,
        width: image.width,
        height: image.height,
        rgbaBase64: image.rgbaBase64,
        format: config.sprayFormat,
        libraryDir: config.libraryDir,
      });
      await refresh();
      selectSpray(created.id);
      toast.success(t("import.success", { name: created.name }));
    } catch (err) {
      toast.error(toMessage(err));
    }
  };

  const doDelete = async (spray: Spray, alsoRemoveFromGame: boolean) => {
    try {
      await container.scanSprays.delete(spray.vtfPath, spray.vmtPath);
      if (alsoRemoveFromGame && destinationDir) {
        const base = `${destinationDir}/${spray.name}`;
        await container.scanSprays.delete(`${base}.vtf`, `${base}.vmt`);
      }
      if (selectedId === spray.id) selectSpray(null);
      await refresh();
      await refreshApplied();
      toast.success(t("delete.success", { name: spray.name }));
    } catch (err) {
      toast.error(toMessage(err));
    }
  };

  const onDrop = (e: React.DragEvent) => {
    e.preventDefault();
    setDragging(false);
    const file = e.dataTransfer.files?.[0];
    if (!file) return;
    if (file.type === "image/png") {
      void handleImportFile(file);
    } else {
      toast.error(t("import.notPng"));
    }
  };

  return (
    <div
      className="relative flex h-full flex-col"
      onDragOver={(e) => {
        e.preventDefault();
        setDragging(true);
      }}
      onDragLeave={(e) => {
        if (e.currentTarget === e.target) setDragging(false);
      }}
      onDrop={onDrop}
    >
      {dragging && (
        <div className="pointer-events-none absolute inset-0 z-50 m-2 flex items-center justify-center rounded-lg border-2 border-dashed border-primary bg-background/80 text-sm font-medium text-primary backdrop-blur-sm">
          {t("import.dropHint")}
        </div>
      )}
      <Toolbar
        search={search}
        onSearch={setSearch}
        count={filtered.length}
        canApply={canApply}
        loading={loading}
        onRefresh={refresh}
        onOpenSettings={() => setSettingsOpen(true)}
        onApply={() => requestApply(selectedSpray)}
        onImport={handleImportFile}
      />

      <main className="flex-1 overflow-y-auto">
        {!config.libraryDir ? (
          <EmptyState
            icon={FolderInput}
            title={t("library.noLibrary")}
            hint={t("library.emptyHint")}
            action={
              <Button variant="outline" onClick={() => setSettingsOpen(true)}>
                {t("library.openSettings")}
              </Button>
            }
          />
        ) : loading ? (
          <EmptyState icon={ImageOff} title={t("library.loading")} />
        ) : sprays.length === 0 ? (
          <EmptyState icon={ImageOff} title={t("library.empty")} hint={t("library.emptyHint")} />
        ) : filtered.length === 0 ? (
          <EmptyState icon={SearchX} title={t("library.noResults")} />
        ) : (
          <SprayGrid
            sprays={filtered}
            selectedId={selectedId}
            favorites={config.favorites}
            appliedNames={appliedNames}
            onSelect={selectSpray}
            onActivate={onActivate}
            onToggleFavorite={toggleFavorite}
            onDelete={setPendingDelete}
          />
        )}
      </main>

      <SettingsDialog
        open={settingsOpen}
        onOpenChange={setSettingsOpen}
        config={config}
        games={games}
        detection={detection}
        onSave={save}
      />

      <ApplyConfirmDialog
        open={pendingSpray !== null}
        onOpenChange={(open) => {
          if (!open) setPendingSpray(null);
        }}
        spray={pendingSpray}
        willOverwrite
        willBackup={config.createBackup}
        onConfirm={() => {
          const spray = pendingSpray;
          setPendingSpray(null);
          if (spray) void doApply(spray);
        }}
      />

      <DeleteConfirmDialog
        open={pendingDelete !== null}
        onOpenChange={(open) => {
          if (!open) setPendingDelete(null);
        }}
        spray={pendingDelete}
        applied={pendingDelete ? appliedNames.has(pendingDelete.name.toLowerCase()) : false}
        onConfirm={(alsoRemoveFromGame) => {
          const spray = pendingDelete;
          setPendingDelete(null);
          if (spray) void doDelete(spray, alsoRemoveFromGame);
        }}
      />
    </div>
  );
}
