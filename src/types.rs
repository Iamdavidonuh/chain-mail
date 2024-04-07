#[derive(Debug, serde::Serialize)]
pub(crate) struct GithubCommitInfo {
    pub(crate) title: String,
    pub(crate) verbose_title: String,
    pub(crate) commit_url: String,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct GithubIssues {
    pub(crate) title: String,
    pub(crate) issue_url: String,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct GithubResults {
    pub(crate) name: Option<String>,

    pub(crate) profile_url: Option<String>,
    pub(crate) commit_history: Vec<GithubCommitInfo>,
    pub(crate) issues: Vec<GithubIssues>,
}

impl GithubResults {
    pub(crate) fn new() -> Self {
        Self {
            name: None,
            profile_url: None,
            commit_history: Vec::new(),
            issues: Vec::new(),
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct PinterestResults;

#[derive(Debug, serde::Serialize)]
pub struct ProfileData {
    pub(crate) github: Option<GithubResults>,
    pub(crate) pinterest: Option<PinterestResults>,
}

impl ProfileData {
    pub(crate) fn new() -> Self {
        Self {
            github: None,
            pinterest: None,
        }
    }
}
