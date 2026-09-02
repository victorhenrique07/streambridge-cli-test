use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Issue {
    pub repository_url: String,
    pub id: u64,
    pub title: String,
    pub state: String,
    pub pull_request: Option<serde_json::Value>,
}
