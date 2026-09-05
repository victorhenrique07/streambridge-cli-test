mod application;
mod domain;

use crate::application::cli::Config;
use crate::application::cli::parse_args;
use crate::application::connectors::github_connection;
use crate::domain::issues::Issue;

use std::error::Error;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let total_start = Instant::now();
    let config: Config = parse_args();

    let issues: Vec<Issue> = github_connection(config).await?;

    let total_issues = issues.len();

    let time_lapse = total_start.elapsed().as_secs() / 60;
    
    println!("Total de {total_issues} issues extraidas.");
    println!("Duração: {} minutos", time_lapse);

    Ok(())
}
