use crate::domain::issues::Issue;
use std::collections::HashMap;

use std::error::Error;

pub async fn api_connection(
    items_per_page: u8,
    user: String,
    repo_name: String,
) -> Result<Vec<Issue>, Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let _client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID deve estar definido.");
    let _client_secrets =
        std::env::var("CLIENT_SECRETS").expect("CLIENT_SECRETS deve estar definido.");
    let _personal_access_token =
        std::env::var("PERSONAL_ACCESS_TOKEN").expect("PERSONAL_ACCESS_TOKEN deve estar definido.");

    let custom_user_agent = "CLIRustApp/1.0";

    let _client = reqwest::Client::builder()
        .user_agent(custom_user_agent)
        .build()?;

    let mut issues: HashMap<u64, Issue> = HashMap::new();

    let mut url = format!(
        "https://api.github.com/repos/{user}/{repo_name}/issues?page=1&per_page={items_per_page}"
    );

    loop {
        let response = _client
            .get(&url)
            .basic_auth(
                _client_id.to_string(),
                Some(_client_secrets.clone()),
            )
            .json(&serde_json::json!({
                "access_token": _personal_access_token.to_string()
            }))
            .header("User-Agent", "my-app-name")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        let headers = response.headers().clone();
        let body_text = response.text().await?;

        let result: Result<Vec<Issue>, _> = serde_json::from_str(&body_text);
        if result.is_err() {
            println!("Falha ao analisar o json.");
            eprintln!("Url utilizada: {url}");
        }
        let new_issues = result?;

        for new_issue in new_issues.into_iter() {
            if new_issue.pull_request.is_none() {
                issues.entry(new_issue.id).or_insert(new_issue);
            }
        }

        if let Some(link) = headers.get("link") {
            if let Ok(link_header) = link.to_str() {
                if let Ok(parsed_links) = parse_link_header::parse(link_header) {
                    if let Some(next_link) = parsed_links.get(&Some("next".to_string())) {
                        url = next_link.raw_uri.to_string();

                        println!("{}", url);
                    } else {
                        println!("Essa foi a última página.");
                        break;
                    }
                }
            }
        }

        println!("{}", issues.len());
    }

    let result: Vec<Issue> = issues.into_values().collect();

    Ok(result)
}
