extern crate chrono;

use crate::application::cli::Config;
use crate::domain::issues::Issue;
use core::time;
use chrono::DateTime;
use reqwest::header::HeaderMap;

use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Write};
use std::{env, process, thread};

struct Keys {
    personal_access_token: String,
    repository_name: String,
    repository_user: String,
    api_url: String
}

pub async fn github_connection(config: Config) -> Result<Vec<Issue>, Box<dyn Error>> {
    let mut page: u8 = 0;
    let keys: Keys = get_api_keys();

    let _client = reqwest::Client::builder()
        .user_agent("CLIRustApp/1.0")
        .build()?;

    let mut issues: Vec<Issue> = Vec::new();

    let mut url = format!(
        "https://{}/repos/{}/{}/issues?page=1&per_page={}",
        keys.api_url, keys.repository_user, keys.repository_name, config.items_per_page
    );

    loop {
        let mut response = _client
            .get(&url)
            .bearer_auth(keys.personal_access_token.to_string())
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        if response.status() == 429 {
            let retry_after: i64 = check_rate_limits(response.headers().clone());

            let retry_after = time::Duration::from_secs(retry_after as u64);

            thread::sleep(retry_after);

            response = _client
                .get(&url)
                .bearer_auth(keys.personal_access_token.to_string())
                .header("Accept", "application/vnd.github+json")
                .send()
                .await?;
        } else if response.status() == 403 {
            for attempt in 0..config.retry_attempts {
                let secs = 2 * 2u8.pow(attempt as u32);

                let retry_after = time::Duration::from_secs(secs as u64);

                println!("Status Code: 403");
                println!("Houve um problema na chamada.");
                println!("Tentando novamente em {} segundos.", secs);

                thread::sleep(retry_after);

                response = _client
                    .get(&url)
                    .bearer_auth(keys.personal_access_token.to_string())
                    .header("Accept", "application/vnd.github+json")
                    .send()
                    .await?;
            }

            let retry_after = check_rate_limits(response.headers().clone());
            
            println!("Houve um problema na chamada. Tente novamente em {} minutos.", retry_after);
            process::exit(1);
        } else if response.status() != 200 {
            println!(
                "Ocorreu um problema, tente novamente. Status Code da requisição: {}",
                response.status()
            );
            process::exit(1);
        }

        let headers = response.headers().clone();
        let body_text = response.text().await?;

        let result: Result<Vec<Issue>, _> = serde_json::from_str(&body_text);

        let new_issues = match result {
            Ok(issues) => {
                let mut issues = issues;
                issues.retain(|i| i.pull_request.is_none());
                issues
            }
            Err(error) => panic!("Erro ao separar issues de pull requests. Error: {error}"),
        };

        issues.extend(new_issues.clone());

        if let Some(link) = headers.get("link") {
            if let Ok(link_header) = link.to_str() {
                if let Ok(parsed_links) = parse_link_header::parse(link_header) {
                    if let Some(next_link) = parsed_links.get(&Some("next".to_string())) {
                        url = next_link.raw_uri.to_string();
                    } else {
                        println!("Essa foi a última página.");
                        break;
                    }
                }
            }
        }

        page += 1;
        println!(
            "{}/{} Issues encontradas. Pagina {}",
            new_issues.len(),
            config.items_per_page,
            page
        );
    }

    create_file(keys.repository_user, keys.repository_name, issues.clone())?;

    Ok(issues)
}

fn get_api_keys() -> Keys {
    dotenvy::dotenv().ok();

    let _personal_access_token = match std::env::var("PERSONAL_ACCESS_TOKEN") {
        Ok(value) => value,
        Err(_) => "PERSONAL_ACCESS_TOKEN deve estar definido.".to_string(),
    };

    let _repository_user = match std::env::var("REPOSITORY_USER") {
        Ok(value) => value,
        Err(_) => "REPOSITORY_USER deve estar definido.".to_string(),
    };

    let _repository_name = match std::env::var("REPOSITORY_NAME") {
        Ok(value) => value,
        Err(_) => "REPOSITORY_NAME deve estar definido.".to_string(),
    };

    let _api_url = match std::env::var("API_URL") {
        Ok(value) => {
            let mut value = value.replace("https://", "");

            let splited_url: Vec<&str> = value.split_terminator('/').collect();

            value = splited_url[0].to_string();

            value
        },
        Err(_) => "API_URL deve estar definido.".to_string(),
    };

    let keys = Keys {
        personal_access_token: _personal_access_token,
        repository_name: _repository_name,
        repository_user: _repository_user,
        api_url: _api_url,
    };

    keys
}

fn check_rate_limits(headers: HeaderMap) -> i64 {
    let mut retry_after: i64 = 0;

    if let Some(header_response) = headers.get("Retry-After") {
        let retry_after_str = match header_response.to_str() {
            Ok(res) => res,
            Err(error) => panic!("Não foi possível extrair o Retry-After. Error: {}", error),
        };

        retry_after = match str::parse::<i64>(retry_after_str) {
            Ok(res) => res,
            Err(error) => panic!("Não foi possível converter o timestamp do rate limit. Error: {}", error),
        };
    }

    if let Some(header_response) = headers.get("x-ratelimit-reset") {
        let rate_limit_timestamp_str = match header_response.to_str() {
            Ok(res) => res,
            Err(error) => panic!("Não foi possível extrair o timestamp. Error: {}", error),
        };

        let parsed_timestamp = match str::parse::<u64>(rate_limit_timestamp_str) {
            Ok(res) => res,
            Err(error) => panic!("Não foi possível converter o timestamp do rate limit. Error: {}", error),
        };

        if let Some(datetime) = DateTime::from_timestamp(parsed_timestamp as i64, 0) {
            let now = chrono::Utc::now();

            retry_after = datetime.signed_duration_since(now).num_minutes();
        }
    }

    retry_after
}

fn create_file(user: String, repo_name: String, issues: Vec<Issue>) -> io::Result<()> {
    let mut root_path = env::current_dir()?;

    let file_name = format!("{}-{}-issues.json", user, repo_name);

    root_path.push("issues");

    let file_path = root_path.join(file_name);

    if !root_path.exists() {
        fs::create_dir_all(root_path)?;
    }

    let mut file = File::create(file_path.clone())?;

    writeln!(file, "{} Issues", repo_name)?;

    let issues_json = serde_json::to_string_pretty(&issues)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    file.write_all(issues_json.as_bytes())?;

    println!("Issues salvas em {}", file_path.display());

    Ok(())
}
