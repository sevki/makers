import { Camera, RefreshCw } from "lucide-react";
import * as React from "react";
import { CommandProvider } from "@/components/command-provider/command-provider";
import { JobDetailDialog } from "@/components/job-detail-dialog";
import { JobStatusBadge } from "@/components/job-status-badge";
import { NewJobDialog } from "@/components/new-job-dialog";
import ThemeToggle from "@/components/theme-toggle/theme-toggle";
import { Button } from "@/components/ui/button";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { useJobs } from "@/hooks/use-jobs";
import { ThemeProvider } from "@/lib/theme-context";
import type { Job } from "@/lib/api";

function formatTime(iso: string) {
  return new Date(iso).toLocaleString();
}

const STATUS_ORDER = ["queued", "running", "succeeded", "failed"] as const;

function Dashboard() {
  const { jobs, loading, error, refresh } = useJobs();
  const [selected, setSelected] = React.useState<Job | null>(null);

  const runningCount = jobs.filter((j) => j.status === "running").length;
  const statusCounts = React.useMemo(() => {
    const counts = { queued: 0, running: 0, succeeded: 0, failed: 0 };
    for (const job of jobs) counts[job.status] += 1;
    return counts;
  }, [jobs]);

  return (
    <div className="mx-auto max-w-5xl px-6 py-10">
      <header className="mb-8 flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-semibold tracking-tight">make-srv</h1>
          <p className="text-sm text-muted-foreground">
            {jobs.length} job{jobs.length === 1 ? "" : "s"}
            {runningCount > 0 ? ` · ${runningCount} running now` : ""}
          </p>
        </div>
        <div className="flex items-center gap-2">
          <Button
            variant="outline"
            size="icon"
            aria-label="Refresh jobs"
            disabled={loading}
            onClick={() => refresh()}
          >
            <RefreshCw className={loading ? "h-4 w-4 animate-spin" : "h-4 w-4"} />
          </Button>
          <NewJobDialog />
          <ThemeToggle />
        </div>
      </header>

      {jobs.length > 0 && (
        <div className="mb-6 flex gap-4 text-sm text-muted-foreground">
          {STATUS_ORDER.map((status) => (
            <span key={status}>
              {statusCounts[status]} {status}
            </span>
          ))}
        </div>
      )}

      {error && (
        <p className="mb-4 rounded-md bg-destructive/10 p-3 text-sm text-destructive">
          Failed to reach make-srv: {error}
        </p>
      )}

      <div className="rounded-lg border">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Repository</TableHead>
              <TableHead>Status</TableHead>
              <TableHead>Routes</TableHead>
              <TableHead>Created</TableHead>
              <TableHead className="text-right">Comment</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {!loading && jobs.length === 0 && (
              <TableRow>
                <TableCell colSpan={5} className="py-12 text-center">
                  <div className="flex flex-col items-center gap-2 text-muted-foreground">
                    <Camera className="h-6 w-6" />
                    <p className="text-sm">
                      No jobs yet. Enqueue one, or wait for CI to submit one from a pull request.
                    </p>
                  </div>
                </TableCell>
              </TableRow>
            )}
            {jobs.map((job) => (
              <TableRow
                key={job.id}
                className="cursor-pointer"
                onClick={() => setSelected(job)}
              >
                <TableCell className="font-mono text-sm">
                  {job.repo}#{job.pr_number}
                </TableCell>
                <TableCell>
                  <JobStatusBadge status={job.status} />
                </TableCell>
                <TableCell className="text-sm text-muted-foreground">
                  {job.routes.length}
                </TableCell>
                <TableCell className="text-sm text-muted-foreground">
                  {formatTime(job.created_at)}
                </TableCell>
                <TableCell className="text-right">
                  {job.comment_url ? (
                    <a
                      href={job.comment_url}
                      target="_blank"
                      rel="noreferrer"
                      onClick={(e) => e.stopPropagation()}
                      className="text-sm text-primary underline underline-offset-4"
                    >
                      view
                    </a>
                  ) : (
                    <span className="text-sm text-muted-foreground">&mdash;</span>
                  )}
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>

      <JobDetailDialog job={selected} onOpenChange={(open) => !open && setSelected(null)} />
    </div>
  );
}

function App() {
  return (
    <ThemeProvider defaultTheme="system" storageKey="make-srv-theme">
      <CommandProvider>
        <Dashboard />
      </CommandProvider>
    </ThemeProvider>
  );
}

export default App;
