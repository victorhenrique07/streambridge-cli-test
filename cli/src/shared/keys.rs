use crate::shared::errors::Errors;

pub struct Keys {
    pub personal_access_token: String,
    pub api_url: String,
    pub issues_path_target: String,
}

pub fn get_api_keys() -> Keys {
    dotenvy::dotenv().ok();

    let _personal_access_token = match std::env::var("PERSONAL_ACCESS_TOKEN") {
        Ok(value) => value,
        Err(_) => panic!(
            "{}",
            Errors::EnvironmentVariableMissingError("PERSONAL_ACCESS_TOKEN".to_string())
        ),
    };

    let _api_url = match std::env::var("API_URL") {
        Ok(value) => {
            let mut value = value.replace("https://", "");

            let splited_url: Vec<&str> = value.split_terminator('/').collect();

            value = splited_url[0].to_string();

            value
        }
        Err(_) => panic!(
            "{}",
            Errors::EnvironmentVariableMissingError("API_URL".to_string())
        ),
    };

    let _directory_target = match std::env::var("ISSUES_PATH_TARGET") {
        Ok(value) => value,
        Err(_) => panic!(
            "{}",
            Errors::EnvironmentVariableMissingError("DIRECTORY_TARGET".to_string())
        ),
    };

    let keys = Keys {
        personal_access_token: _personal_access_token,
        api_url: _api_url,
        issues_path_target: _directory_target,
    };

    keys
}
