use crate::domain::issues::Issue;

use std::error::Error;


pub async fn api_connection(items_per_page: u8) -> Result<Vec<Issue>, Box<dyn Error>> {
    let custom_user_agent = "CLIRustApp/1.0";

    let client = reqwest::Client::builder()
        .user_agent(custom_user_agent)
        .build()?;

    let page_index: u8 = 1;
    
    let mut posts: Vec<Issue> = Vec::new();

    let url = format!(
    "https://api.github.com/repos/octocat/Spoon-Knife/issues?page={page_index}&per_page={items_per_page}");

    while page_index <= 1 {
        let response = client
            .get(&url)
            .basic_auth("ID", Some("secret")) 
            .json(&serde_json::json!({
                "access_token": "access_token"
            }))
            .send()
            .await?;

        posts = response.json().await?;
    }
    
    let result = posts;

    Ok(result)
}