export type JobStatus = "queued" | "running" | "succeeded" | "failed";

export interface Screenshot {
  route: string;
  file_name: string;
  raw_url: string | null;
}

export interface Job {
  id: string;
  repo: string;
  pr_number: number;
  head_sha: string;
  base_url: string;
  routes: string[];
  status: JobStatus;
  screenshots: Screenshot[];
  comment_url: string | null;
  error: string | null;
  created_at: string;
  updated_at: string;
}

export interface NewJobInput {
  repo: string;
  pr_number: number;
  head_sha: string;
  base_url: string;
  routes: string[];
}

async function asJson<T>(response: Response): Promise<T> {
  if (!response.ok) {
    const body = await response.text();
    throw new Error(`${response.status} ${response.statusText}: ${body}`);
  }
  return response.json() as Promise<T>;
}

export const api = {
  listJobs: (): Promise<Job[]> => fetch("/api/jobs").then((r) => asJson(r)),

  getJob: (id: string): Promise<Job> => fetch(`/api/jobs/${id}`).then((r) => asJson(r)),

  createJob: (input: NewJobInput): Promise<Job> =>
    fetch("/api/jobs", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify(input),
    }).then((r) => asJson(r)),

  /** Subscribes to server-sent job updates; returns an unsubscribe function. */
  subscribe: (onJob: (job: Job) => void): (() => void) => {
    const source = new EventSource("/api/events");
    source.addEventListener("job", (event) => {
      onJob(JSON.parse((event as MessageEvent).data) as Job);
    });
    return () => source.close();
  },
};
