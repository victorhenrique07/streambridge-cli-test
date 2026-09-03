use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Issue {
    pub id: u64,
    pub html_url: String,
    pub repository_url: String,
    pub title: String,
    pub state: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<serde_json::Value>,
}
