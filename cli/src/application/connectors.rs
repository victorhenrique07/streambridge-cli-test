extern crate chrono;

use crate::application::cli::Config;
use crate::domain::issues::Issue;
use crate::shared::errors::Errors;
use crate::shared::keys::{Keys, get_api_keys};
use crate::shared::watermark::{Watermark, check_watermark, create_watermark};
use chrono::DateTime;
use core::time;
use reqwest::Response;
use reqwest::header::HeaderMap;
use std::path::PathBuf;

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::{process, thread};

pub async fn github_connection(config: Config) -> Result<Vec<Issue>, Errors> {
    let mut page: u8 = 0;
    let keys: Keys = get_api_keys();

    let client = reqwest::Client::builder()
        .timeout(time::Duration::from_secs(10))
        .user_agent("CLIRustApp/1.0")
        .build()?;

    let mut issues: Vec<Issue> = Vec::new();

    let mut url = format!(
        "https://{}/repos/{}/{}/issues?page=1&per_page={}",
        keys.api_url, config.repository_user, config.repository_name, config.items_per_page
    );

    loop {
        let watermark = check_watermark();

        let mut response = get_issues(&client, &keys.personal_access_token, &url, &watermark).await;

        if response.status() == 429 {
            let retry_after: i64 = check_rate_limits(&response.headers());

            let retry_after = time::Duration::from_secs(retry_after as u64);

            thread::sleep(retry_after);

            response = get_issues(&client, &keys.personal_access_token, &url, &watermark).await;
        } else if response.status() == 403 {
            for attempt in 0..config.retry_attempts {
                let secs = 2 * 2u8.pow(attempt as u32);

                let retry_after = time::Duration::from_secs(secs as u64);

                println!("Status Code: 403");
                println!("Houve um problema na chamada.");
                println!("Tentando novamente em {} segundos.", secs);

                thread::sleep(retry_after);

                response = get_issues(&client, &keys.personal_access_token, &url, &watermark).await;

                if response.status() == 200 {
                    break;
                }
            }

            let retry_after = check_rate_limits(response.headers());

            println!("{}", Errors::RateLimitError(retry_after));
            process::exit(1);
        } else if response.status() != 200 {
            println!("{}", Errors::RequestError);
            process::exit(1);
        }

        let headers = response.headers().clone();
        let body_text = response.text().await?;

        let result: Result<Vec<Issue>, _> = serde_json::from_str(&body_text);

        let new_issues = match result {
            Ok(issues) => {
                let mut issues = issues;
                issues.retain(|i| i.pull_request.is_none());

                if let Some(wm) = &watermark {
                    issues.retain(|i| i.id != wm.issue_id);
                }

                issues
            }
            Err(error) => panic!("{}", Errors::FetchIssuesFailed(error)),
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
        } else {
            break;
        }

        page += 1;
        println!(
            "{}/{} Issues encontradas. Pagina {}",
            new_issues.len(),
            config.items_per_page,
            page
        );
    }

    create_file(
        config.repository_user,
        config.repository_name,
        keys.issues_path_target,
        &mut issues,
    )?;

    Ok(issues)
}

fn check_rate_limits(headers: &HeaderMap) -> i64 {
    let mut retry_after: i64 = 0;

    if let Some(header_response) = headers.get("retry-after") {
        let retry_after_str = match header_response.to_str() {
            Ok(res) => res,
            Err(error) => panic!("{}", &Errors::TimestampExtractError(error)),
        };

        retry_after = match str::parse::<i64>(retry_after_str) {
            Ok(res) => res,
            Err(error) => panic!("{}", Errors::TimestampParseError(error)),
        };
    }

    if let Some(header_response) = headers.get("x-ratelimit-reset") {
        let rate_limit_timestamp_str = match header_response.to_str() {
            Ok(res) => res,
            Err(error) => panic!("{}", &Errors::TimestampExtractError(error)),
        };

        let parsed_timestamp = match str::parse::<u64>(rate_limit_timestamp_str) {
            Ok(res) => res,
            Err(error) => panic!("{}", Errors::TimestampParseError(error)),
        };

        if let Some(datetime) = DateTime::from_timestamp(parsed_timestamp as i64, 0) {
            let now = chrono::Utc::now();

            retry_after = datetime.signed_duration_since(now).num_minutes();
        }
    }

    retry_after
}

async fn get_issues(
    client: &reqwest::Client,
    personal_access_token: &String,
    url: &String,
    since: &Option<Watermark>,
) -> Response {
    let mut request = client
        .get(url)
        .bearer_auth(personal_access_token)
        .header("Accept", "application/vnd.github+json");

    if let Some(s) = since {
        request = request.query(&[("since", s.issue_updated_at)]);
    }

    let response = match request.send().await {
        Ok(resp) => resp,
        Err(e) if e.is_timeout() => {
            panic!("{}", Errors::TimeoutError(e.to_string()));
        }
        Err(e) => {
            panic!("Erro na requisição: {}", e)
        }
    };

    response
}

fn create_file(
    user: String,
    repo_name: String,
    root_path: String,
    issues: &mut Vec<Issue>,
) -> io::Result<()> {
    issues.sort_by_key(|x| x.updated_at);
    issues.reverse();

    if issues.is_empty() {
        return Ok(());
    }

    let _watermark = create_watermark(Watermark {
        issue_id: issues[0].id,
        issue_updated_at: issues[0].updated_at,
    });

    let root_path = std::path::PathBuf::from(root_path);

    let file_name = format!("{}-{}-issues.json", user, repo_name);

    let file_path = root_path.join(file_name);

    if !root_path.exists() {
        fs::create_dir_all(&root_path)?;
    }

    if file_path.exists() {
        append_issues(issues.clone(), &file_path);
    }

    File::create(file_path.clone())?;

    let mut file = match OpenOptions::new().write(true).open(&file_path) {
        Ok(value) => value,
        Err(_) => panic!("dada"),
    };

    let issues_json = match serde_json::to_string_pretty(&issues) {
        Ok(issues) => issues,
        Err(_) => panic!("{}", &Errors::SerializingError()),
    };

    file.write_all(issues_json.as_bytes())?;

    println!("Issues salvas em {}", file_path.display());

    Ok(())
}

fn append_issues(issues: Vec<Issue>, file_path: &PathBuf) -> Vec<Issue> {
    let mut issues = issues;
    let mut file = match File::open(&file_path) {
        Ok(value) => value,
        Err(_) => panic!(),
    };

    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    let data: Vec<Issue> = match serde_json::from_str(&content) {
        Ok(value) => value,
        Err(_) => panic!("{}", &Errors::SerializingError()),
    };

    issues.extend(data);

    println!("total de {} issues agora.", issues.len());
    issues
}
