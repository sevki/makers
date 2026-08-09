import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { JobStatusBadge } from "@/components/job-status-badge";
import type { Job } from "@/lib/api";

interface JobDetailDialogProps {
  job: Job | null;
  onOpenChange: (open: boolean) => void;
}

export function JobDetailDialog({ job, onOpenChange }: JobDetailDialogProps) {
  return (
    <Dialog open={job !== null} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-xl">
        {job && (
          <>
            <DialogHeader>
              <DialogTitle className="flex items-center gap-2 font-mono text-base">
                {job.repo}#{job.pr_number}
                <JobStatusBadge status={job.status} />
              </DialogTitle>
              <DialogDescription>
                job {job.id} &middot; commit {job.head_sha.slice(0, 12)}
              </DialogDescription>
            </DialogHeader>

            <ScrollArea className="max-h-[60vh]">
              <div className="space-y-4 pr-4">
                {job.error && (
                  <p className="rounded-md bg-destructive/10 p-3 text-sm text-destructive">
                    {job.error}
                  </p>
                )}

                {job.comment_url && (
                  <a
                    href={job.comment_url}
                    target="_blank"
                    rel="noreferrer"
                    className="text-sm text-primary underline underline-offset-4"
                  >
                    View PR comment
                  </a>
                )}

                <Separator />

                <div className="space-y-3">
                  {job.screenshots.length === 0 && (
                    <p className="text-sm text-muted-foreground">No screenshots yet.</p>
                  )}
                  {job.screenshots.map((shot) => (
                    <figure key={shot.route} className="space-y-1">
                      <figcaption className="font-mono text-xs text-muted-foreground">
                        {shot.route}
                      </figcaption>
                      {shot.raw_url ? (
                        <img
                          src={shot.raw_url}
                          alt={shot.route}
                          className="w-full rounded-md border"
                        />
                      ) : (
                        <p className="text-sm text-muted-foreground">{shot.file_name}</p>
                      )}
                    </figure>
                  ))}
                </div>
              </div>
            </ScrollArea>
          </>
        )}
      </DialogContent>
    </Dialog>
  );
}
