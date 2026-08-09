use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Screenshot {
    pub route: String,
    pub file_name: String,
    pub raw_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub repo: String,
    pub pr_number: u64,
    pub head_sha: String,
    pub base_url: String,
    pub routes: Vec<String>,
    pub status: JobStatus,
    pub screenshots: Vec<Screenshot>,
    pub comment_url: Option<String>,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewJobRequest {
    pub repo: String,
    pub pr_number: u64,
    pub head_sha: String,
    pub base_url: String,
    #[serde(default = "default_routes")]
    pub routes: Vec<String>,
}

fn default_routes() -> Vec<String> {
    vec!["/".to_string()]
}

impl Job {
    pub fn new(req: NewJobRequest) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            repo: req.repo,
            pr_number: req.pr_number,
            head_sha: req.head_sha,
            base_url: req.base_url,
            routes: req.routes,
            status: JobStatus::Queued,
            screenshots: Vec::new(),
            comment_url: None,
            error: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn route_file_name(route: &str) -> String {
        let slug = if route == "/" {
            "root".to_string()
        } else {
            route.trim_matches('/').replace('/', "-")
        };
        format!("{slug}.png")
    }
}
