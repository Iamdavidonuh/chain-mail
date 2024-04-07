use async_trait::async_trait;
use headless_chrome::Browser;
use std::process;
use std::str::FromStr;

use crate::types::ProfileData;

pub mod github;
pub mod pinterest;

#[async_trait]
pub(crate) trait Buildrequest {
    async fn search(&self, result: &mut ProfileData) -> Result<(), anyhow::Error>;

    fn parse_url(&self, url: &str, query_params: Option<&[(&str, &str)]>) -> reqwest::Url {
        if let Some(params) = query_params {
            let url = reqwest::Url::parse_with_params(url, params).expect("cannot parse url");
            return url;
        }
        reqwest::Url::from_str(url).expect("Failed to create url")
    }
    async fn get_request(&self, url: reqwest::Url) -> reqwest::Response {
        let client = reqwest::Client::new();

        let response = client.get(url).send().await;

        match response {
            Ok(response) => response,
            Err(error) => {
                eprintln!("error making request: {error}");
                process::exit(1);
            }
        }
    }
    async fn run_browser(&self, url: reqwest::Url) -> Result<String, anyhow::Error> {
        let browser = Browser::default()?;

        let tab = browser.new_tab()?;
        // Navigate to url
        tab.navigate_to(url.as_str())?;
        tab.wait_until_navigated()?;

        let content = tab.get_content()?;

        Ok(content)
    }
}

pub enum Engines {
    PINTEREST,
    GITHUB,
}

impl TryFrom<String> for Engines {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "pinterest" => Ok(Self::PINTEREST),
            "github" => Ok(Self::GITHUB),

            _other => Err(format!("Unsupported Engine type: {}", &value)),
        }
    }
}
