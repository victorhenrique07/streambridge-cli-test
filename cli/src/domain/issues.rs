use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Issue {
    repository_url: String,
    id: u64,
    title: String,
    state: String,
}