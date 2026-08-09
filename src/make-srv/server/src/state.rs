use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use tokio::sync::{Semaphore, broadcast};
use uuid::Uuid;

use crate::config::Config;
use crate::job::{Job, JobStatus, NewJobRequest};
use crate::{github, screenshot};

/// Broadcast to every SSE subscriber whenever a job changes state.
#[derive(Debug, Clone, serde::Serialize)]
pub struct JobEvent {
    pub job: Job,
}

pub struct AppState {
    pub config: Config,
    pub jobs: tokio::sync::RwLock<HashMap<Uuid, Job>>,
    pub events: broadcast::Sender<JobEvent>,
    /// Bounds how many jobs run at once; excess jobs simply wait for a permit,
    /// which is what lets the server accept many jobs while still capping
    /// concurrency.
    semaphore: Arc<Semaphore>,
    http: reqwest::Client,
}

impl AppState {
    pub fn new(config: Config) -> Arc<Self> {
        let (events, _) = broadcast::channel(256);
        Arc::new(Self {
            semaphore: Arc::new(Semaphore::new(config.max_concurrent_jobs.max(1))),
            config,
            jobs: tokio::sync::RwLock::new(HashMap::new()),
            events,
            http: reqwest::Client::new(),
        })
    }

    pub async fn list_jobs(&self) -> Vec<Job> {
        let jobs = self.jobs.read().await;
        let mut jobs: Vec<Job> = jobs.values().cloned().collect();
        jobs.sort_by_key(|job| std::cmp::Reverse(job.created_at));
        jobs
    }

    pub async fn get_job(&self, id: Uuid) -> Option<Job> {
        self.jobs.read().await.get(&id).cloned()
    }

    /// Records a new queued job and spawns its execution in the background so
    /// that submitting several jobs in a row runs them concurrently (bounded
    /// by `MAX_CONCURRENT_JOBS`) instead of one after another.
    pub async fn enqueue(self: &Arc<Self>, req: NewJobRequest) -> Job {
        let job = Job::new(req);
        self.jobs.write().await.insert(job.id, job.clone());
        self.notify(&job);

        let state = Arc::clone(self);
        let id = job.id;
        tokio::spawn(async move {
            state.run(id).await;
        });

        job
    }

    async fn run(self: Arc<Self>, id: Uuid) {
        let _permit = self
            .semaphore
            .clone()
            .acquire_owned()
            .await
            .expect("semaphore closed");
        self.set_status(id, JobStatus::Running, None).await;

        let Some(job) = self.get_job(id).await else {
            return;
        };

        if self.config.dry_run {
            tokio::time::sleep(Duration::from_millis(200)).await;
            self.mutate(id, |job| {
                job.status = JobStatus::Succeeded;
            })
            .await;
            return;
        }

        let screenshots = match screenshot::capture(&self.config, &job).await {
            Ok(screenshots) => screenshots,
            Err(err) => {
                tracing::error!(job_id = %id, error = %err, "screenshot capture failed");
                self.fail(id, err.to_string()).await;
                return;
            }
        };

        // Record the screenshots as soon as they exist so the dashboard (and
        // this job's record) reflects them even if publishing to GitHub
        // subsequently fails.
        self.mutate(id, |job| {
            job.screenshots = screenshots.clone();
        })
        .await;

        if self.config.github_token.is_none() {
            tracing::warn!(
                "GITHUB_TOKEN not set; captured screenshots but skipped posting a PR comment"
            );
            self.mutate(id, |job| {
                job.status = JobStatus::Succeeded;
            })
            .await;
            return;
        }

        match github::publish(&self.http, &self.config, &job, &screenshots).await {
            Ok(comment_url) => {
                self.mutate(id, |job| {
                    job.status = JobStatus::Succeeded;
                    job.comment_url = Some(comment_url);
                })
                .await;
            }
            Err(err) => {
                tracing::error!(job_id = %id, error = %err, "publishing screenshots failed");
                self.fail(id, err.to_string()).await;
            }
        }
    }

    async fn set_status(&self, id: Uuid, status: JobStatus, error: Option<String>) {
        self.mutate(id, |job| {
            job.status = status;
            job.error = error;
        })
        .await;
    }

    async fn fail(&self, id: Uuid, error: String) {
        self.mutate(id, |job| {
            job.status = JobStatus::Failed;
            job.error = Some(error);
        })
        .await;
    }

    async fn mutate(&self, id: Uuid, f: impl FnOnce(&mut Job)) {
        let mut jobs = self.jobs.write().await;
        if let Some(job) = jobs.get_mut(&id) {
            f(job);
            job.updated_at = Utc::now();
            self.notify(job);
        }
    }

    fn notify(&self, job: &Job) {
        // No subscribers is a normal state (e.g. no dashboard open); ignore.
        let _ = self.events.send(JobEvent { job: job.clone() });
    }
}
