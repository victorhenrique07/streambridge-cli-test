mod application;
mod domain;

use crate::application::cli::Config;
use crate::application::cli::parse_args;
use crate::application::connectors::api_connection;
use crate::domain::issues::Issue;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = parse_args();

    let issues: Vec<Issue> = api_connection(
        config.items_per_page,
        config.repository_user,
        config.repository_name,
    )
    .await?;

    let total_issues = issues.len();

    println!("Executado com sucesso!");
    println!("um total de {total_issues}");

    Ok(())
}

// https://api.github.com/repos/octocat/Spoon-Knife/issues
