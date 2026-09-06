use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Watermark {
    pub issue_id: u64,
    pub issue_updated_at: DateTime<Utc>,
}
