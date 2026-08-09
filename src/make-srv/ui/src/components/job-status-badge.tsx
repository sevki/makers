import { Badge } from "@/components/ui/badge";
import type { JobStatus } from "@/lib/api";
import { cn } from "@/lib/utils";

const STYLES: Record<JobStatus, string> = {
  queued: "bg-muted text-muted-foreground",
  running: "bg-blue-500/15 text-blue-600 dark:text-blue-400",
  succeeded: "bg-emerald-500/15 text-emerald-600 dark:text-emerald-400",
  failed: "bg-destructive/15 text-destructive",
};

const LABELS: Record<JobStatus, string> = {
  queued: "Queued",
  running: "Running",
  succeeded: "Succeeded",
  failed: "Failed",
};

export function JobStatusBadge({ status }: { status: JobStatus }) {
  return (
    <Badge variant="outline" className={cn("border-transparent font-medium", STYLES[status])}>
      {status === "running" && (
        <span className="mr-1.5 inline-block size-1.5 animate-pulse rounded-full bg-current" />
      )}
      {LABELS[status]}
    </Badge>
  );
}
