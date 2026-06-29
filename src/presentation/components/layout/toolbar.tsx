import { RefreshCw, Search, Send, Settings as SettingsIcon } from "lucide-react";
import { useTranslation } from "react-i18next";

import { Button } from "@/presentation/components/ui/button";
import { Input } from "@/presentation/components/ui/input";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/presentation/components/ui/tooltip";

interface ToolbarProps {
  search: string;
  onSearch: (value: string) => void;
  count: number;
  canApply: boolean;
  loading: boolean;
  onRefresh: () => void;
  onOpenSettings: () => void;
  onApply: () => void;
}

export function Toolbar({
  search,
  onSearch,
  count,
  canApply,
  loading,
  onRefresh,
  onOpenSettings,
  onApply,
}: ToolbarProps) {
  const { t } = useTranslation();

  return (
    <header className="flex items-center gap-2 border-b border-border bg-card/60 px-3 py-2 backdrop-blur">
      <div className="flex items-center gap-2 pr-1">
        <img src="/logo.svg" alt="" className="size-6" />
        <span className="text-sm font-semibold tracking-tight">{t("app.title")}</span>
      </div>

      <div className="relative ml-2 flex-1">
        <Search className="pointer-events-none absolute left-2.5 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" />
        <Input
          value={search}
          onChange={(e) => onSearch(e.target.value)}
          placeholder={t("toolbar.search")}
          className="pl-8"
          aria-label={t("toolbar.search")}
        />
      </div>

      <span className="hidden whitespace-nowrap px-1 text-xs text-muted-foreground sm:block">
        {t("toolbar.sprayCount", { count })}
      </span>

      <Tooltip>
        <TooltipTrigger asChild>
          <Button variant="outline" size="icon" onClick={onRefresh} disabled={loading}>
            <RefreshCw className={loading ? "animate-spin" : ""} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>{t("toolbar.refresh")}</TooltipContent>
      </Tooltip>

      <Tooltip>
        <TooltipTrigger asChild>
          <Button variant="outline" size="icon" onClick={onOpenSettings}>
            <SettingsIcon />
          </Button>
        </TooltipTrigger>
        <TooltipContent>{t("toolbar.settings")}</TooltipContent>
      </Tooltip>

      <Button onClick={onApply} disabled={!canApply}>
        <Send />
        {t("toolbar.apply")}
      </Button>
    </header>
  );
}
