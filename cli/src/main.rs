mod application;
mod domain;

use crate::application::connectors::api_connection;
use crate::application::cli::parse_args;
use crate::application::cli::Config;
use crate::domain::issues::Issue;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = parse_args();

    let issues: Vec<Issue> = api_connection(config.items_per_page).await?;

    for issue in issues {
        println!("{:#?}", issue);
    }

    println!("Executado com sucesso!");

    Ok(())
}

// https://api.github.com/repos/octocat/Spoon-Knife/issues