use anyhow::{Error, Ok};
use async_trait::async_trait;
use scraper::{Html, Selector};

use crate::types::{GithubCommitInfo, GithubIssues, GithubResults, ProfileData};

use crate::Buildrequest;

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
            .map(|href| href.text().collect::<String>().trim().to_owned())
            .unwrap();

        let href = user_box
            .select(&Selector::parse("a").unwrap())
            .next()
            .map(|href| href.value().attr("href").unwrap())
            .map(str::to_owned)
            .unwrap();
        let href = format!("{}{}", &self.base_url, href);
        search_result.name = Some(name);
        search_result.profile_url = Some(href);

        Ok(())
    }

    async fn parse_commits(&self, search_result: &mut GithubResults) -> Result<(), anyhow::Error> {
        let query_params = [("q", &self.search_query as &str), ("type", "commits")];

        let search_url = format!("{}/search", &self.base_url);
        let url = self.parse_url(&search_url, Some(&query_params));

        let html_text = self.run_browser(url).await?;
        let document = Html::parse_document(&html_text);

        let commit_container = Selector::parse(r#"h3.Box-sc-g0xbh4-0.eYhAUV"#).unwrap();
        let commit_boxes = document.select(&commit_container);
        let commit_span_ele = Selector::parse(r#".search-match"#).unwrap();

        if commit_boxes.clone().next().is_none() {
            return Ok(());
        }

        let result = commit_boxes
            .filter_map(|ele| {
                let entry = ele.select(&commit_span_ele).next().unwrap();
                let href = entry
                    .select(&Selector::parse("a").unwrap())
                    .next()
                    .map(|href| href.value().attr("href").unwrap())
                    .map(str::to_owned)
                    .unwrap();

                let href = format!("{}{}", &self.base_url, href);

                let name = entry
                    .select(&Selector::parse("a").unwrap())
                    .next()
                    .and_then(|href| Some(href.text().collect::<String>().trim().to_owned()))
                    .unwrap();
                let full_commit = entry
                    .select(&Selector::parse("a").unwrap())
                    .next()
                    .map(|href| href.value().attr("title").unwrap())
                    .map(str::to_owned)
                    .unwrap();

                Some(GithubCommitInfo {
                    title: name,
                    commit_url: href,
                    verbose_title: full_commit,
                })
            })
            .collect();
        search_result.commit_history = result;

        Ok(())
    }

    async fn parse_issues(&self, search_result: &mut GithubResults) -> Result<(), anyhow::Error> {
        let query_params = [("q", &self.search_query as &str), ("type", "issues")];

        let search_url = format!("{}/search", &self.base_url);
        let url = self.parse_url(&search_url, Some(&query_params));

        let html_text = self.run_browser(url).await?;

        let document = Html::parse_document(&html_text);

        let issues_selector = Selector::parse(r#"div.Box-sc-g0xbh4-0.hDWxXB"#).unwrap();
        let mut div_box = document.select(&issues_selector);
        let hrefs_container = Selector::parse(".search-title").unwrap();

        let issue_boxes = div_box.next().unwrap().select(&hrefs_container);

        if issue_boxes.clone().next().is_none() {
            return Ok(());
        }

        let issues: Vec<_> = issue_boxes
            .filter_map(|issue_box| {
                let issue_name = issue_box
                    .select(&Selector::parse("a").unwrap())
                    .next()
                    .map(|href| href.text().collect::<String>().trim().to_owned())
                    .unwrap();

                let issue_href = issue_box
                    .select(&Selector::parse("a").unwrap())
                    .next()
                    .map(|href| href.value().attr("href").unwrap())
                    .map(str::to_owned)
                    .unwrap();
                let issue_href = format!("{}{}", &self.base_url, issue_href);

                println!("{}, {}", issue_href, issue_name);

                Some(GithubIssues {
                    title: issue_name,
                    issue_url: issue_href,
                })
            })
            .collect();

        search_result.issues = issues;

        Ok(())
    }
}
#[async_trait]
impl Buildrequest for Github {
    async fn search(&self, result: &mut ProfileData) -> Result<(), Error> {
        let mut github_results = GithubResults::new();
        // parser user
        if let Err(err) = self.parse_user(&mut github_results).await {
            anyhow::bail!("error fetching git profile info: {}", err);
        };
        // parse commits
        if let Err(err) = self.parse_commits(&mut github_results).await {
            anyhow::bail!("error fetching recent commits: {}", err);
        };

        // parse issues
        if let Err(err) = self.parse_issues(&mut github_results).await {
            anyhow::bail!("error fetching recent issues: {}", err);
        };
        result.github = Some(github_results);
        Ok(())
    }
}
