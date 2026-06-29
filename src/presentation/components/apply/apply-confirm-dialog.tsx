import { AlertTriangle } from "lucide-react";
import { useTranslation } from "react-i18next";

import type { Spray } from "@/domain/entities/spray";
import { Button } from "@/presentation/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/presentation/components/ui/dialog";

interface ApplyConfirmDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  spray: Spray | null;
  willOverwrite: boolean;
  willBackup: boolean;
  onConfirm: () => void;
}

export function ApplyConfirmDialog({
  open,
  onOpenChange,
  spray,
  willOverwrite,
  willBackup,
  onConfirm,
}: ApplyConfirmDialogProps) {
  const { t } = useTranslation();

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>{t("apply.title")}</DialogTitle>
          <DialogDescription>{t("apply.message", { name: spray?.name ?? "" })}</DialogDescription>
        </DialogHeader>

        {willOverwrite && (
          <div className="flex items-start gap-2 rounded-md border border-destructive/40 bg-destructive/10 p-3 text-sm">
            <AlertTriangle className="mt-0.5 size-4 shrink-0 text-destructive" />
            <div>
              <p>{t("apply.overwriteWarning")}</p>
              {willBackup && (
                <p className="text-xs text-muted-foreground">{t("apply.backupNote")}</p>
              )}
            </div>
          </div>
        )}

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            {t("apply.cancel")}
          </Button>
          <Button onClick={onConfirm}>{t("apply.confirm")}</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
