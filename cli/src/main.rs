mod application;
mod domain;

use crate::application::cli::Config;
use crate::application::cli::parse_args;
use crate::application::connectors::github_connection;
use crate::domain::issues::Issue;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = parse_args();

    let issues: Vec<Issue> = github_connection(config).await?;

    let total_issues = issues.len();

    println!("Um total de {total_issues} issues.");

    Ok(())
}
