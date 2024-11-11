use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub github_token: Option<String>,
}
