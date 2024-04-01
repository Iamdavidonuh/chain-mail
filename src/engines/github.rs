use anyhow::Error;
use async_trait::async_trait;

use scraper::{Html, Selector};

use super::Buildrequest;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct GithubResults {
    pub(crate) name: Option<String>,

    pub(crate) profile_url: Option<String>,
}

impl GithubResults {
    fn new() -> Self {
        Self {
            name: None,
            profile_url: None,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Github {
    base_url: String,
    search_query: String,
}
impl Github {
    pub(crate) fn new(email: &str) -> Self {
        Self {
            base_url: "https://www.github.com".to_owned(),
            search_query: email.to_owned(),
        }
    }

    async fn parse_user(&self, search_result: &mut GithubResults) -> Result<(), anyhow::Error> {
        let query_params = [("q", &self.search_query as &str), ("type", "users")];

        let search_url = format!("{}/search", &self.base_url);
        let url = self.parse_url(&search_url, Some(&query_params));
        let html_text = self.run_browser(url).await?;

        let document = Html::parse_document(&html_text);

        let user_selector = Selector::parse(r#"div.Box-sc-g0xbh4-0.hDWxXB"#).unwrap();

        let mut div_box = document.select(&user_selector);

        let hrefs_container = Selector::parse("h3").unwrap();

        if div_box.clone().next().is_none() {
            return Ok(());
        }

        let user_box = div_box
            .next()
            .unwrap()
            .select(&hrefs_container)
            .next()
            .unwrap();
        let name = user_box
            .select(&Selector::parse("a").unwrap())
            .next()
            .and_then(|href| Some(href.text().collect::<String>().trim().to_owned()))
            .unwrap();

        let href = user_box
            .select(&Selector::parse("a").unwrap())
            .next()
            .and_then(|href| href.value().attr("href"))
            .map(str::to_owned)
            .unwrap();
        let href = format!("{}{}", &self.base_url, href);
        search_result.name = Some(name);
        search_result.profile_url = Some(href);

        Ok(())
    }
}
#[async_trait]
impl Buildrequest for Github {
    type Item = GithubResults;
    async fn search(&self) -> Result<Self::Item, Error> {
        let mut github_results = GithubResults::new();
        // parser user
        //

        if let Err(err) = self.parse_user(&mut github_results).await {
            anyhow::bail!("error fetching information: {}", err);
        };
        // TODO: parse commits
        //let url_2 = "https://github.com/search?q=diretnandomnan%40gmail.com&type=commits";
        // parse issues
        // https://github.com/search?q=diretnandomnan%40gmail.com&type=issues
        Ok(github_results)
    }
}
