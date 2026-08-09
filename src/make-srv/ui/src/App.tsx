import * as React from "react";
import { CommandProvider } from "@/components/command-provider/command-provider";
import { JobDetailDialog } from "@/components/job-detail-dialog";
import { JobStatusBadge } from "@/components/job-status-badge";
import { NewJobDialog } from "@/components/new-job-dialog";
import ThemeToggle from "@/components/theme-toggle/theme-toggle";
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

function Dashboard() {
  const { jobs, loading, error } = useJobs();
  const [selected, setSelected] = React.useState<Job | null>(null);

  const runningCount = jobs.filter((j) => j.status === "running").length;

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
          <NewJobDialog />
          <ThemeToggle />
        </div>
      </header>

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
                <TableCell colSpan={5} className="text-center text-muted-foreground">
                  No jobs yet. Enqueue one, or wait for CI to submit one from a pull request.
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
