import * as React from "react";
import { api, type Job } from "@/lib/api";

/**
 * Keeps a live view of every job. Loads the initial list over HTTP, then
 * applies incremental updates pushed over the server's SSE stream so that
 * many jobs progressing at once (queued -> running -> succeeded/failed) show
 * up without re-polling the whole list.
 */
export function useJobs() {
  const [jobs, setJobs] = React.useState<Map<string, Job>>(new Map());
  const [loading, setLoading] = React.useState(true);
  const [error, setError] = React.useState<string | null>(null);

  const refresh = React.useCallback(() => {
    setLoading(true);
    return api
      .listJobs()
      .then((list) => {
        setJobs(new Map(list.map((job) => [job.id, job])));
        setError(null);
      })
      .catch((err: Error) => setError(err.message))
      .finally(() => setLoading(false));
  }, []);

  React.useEffect(() => {
    refresh();

    const unsubscribe = api.subscribe((job) => {
      setJobs((prev) => {
        const next = new Map(prev);
        next.set(job.id, job);
        return next;
      });
    });

    return unsubscribe;
  }, [refresh]);

  const list = React.useMemo(
    () => Array.from(jobs.values()).sort((a, b) => b.created_at.localeCompare(a.created_at)),
    [jobs],
  );

  return { jobs: list, loading, error, refresh };
}
