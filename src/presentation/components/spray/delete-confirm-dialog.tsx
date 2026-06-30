import { Trash2 } from "lucide-react";
import { useEffect, useState } from "react";
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
import { Label } from "@/presentation/components/ui/label";
import { Switch } from "@/presentation/components/ui/switch";

interface DeleteConfirmDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  spray: Spray | null;
  /** Whether this spray currently exists in the game's destination folder. */
  applied: boolean;
  onConfirm: (alsoRemoveFromGame: boolean) => void;
}

export function DeleteConfirmDialog({
  open,
  onOpenChange,
  spray,
  applied,
  onConfirm,
}: DeleteConfirmDialogProps) {
  const { t } = useTranslation();
  const [alsoGame, setAlsoGame] = useState(false);

  // Reset the option each time the dialog opens.
  useEffect(() => {
    if (open) setAlsoGame(false);
  }, [open]);

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>{t("delete.title")}</DialogTitle>
          <DialogDescription>{t("delete.message", { name: spray?.name ?? "" })}</DialogDescription>
        </DialogHeader>

        {applied && (
          <div className="flex items-center justify-between gap-4 rounded-md border border-border p-3">
            <Label htmlFor="alsoGame" className="cursor-pointer font-normal">
              {t("delete.alsoGame")}
            </Label>
            <Switch id="alsoGame" checked={alsoGame} onCheckedChange={setAlsoGame} />
          </div>
        )}

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            {t("delete.cancel")}
          </Button>
          <Button variant="destructive" onClick={() => onConfirm(alsoGame)}>
            <Trash2 />
            {t("delete.confirm")}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
