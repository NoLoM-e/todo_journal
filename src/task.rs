use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    description: String,
    #[serde(with = "ts_seconds")]
    created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(desc: String) -> Task {
        Task {
            description: desc,
            created_at: Utc::now(),
        }
    }
}
