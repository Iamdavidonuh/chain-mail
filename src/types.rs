#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct GithubCommitInfo {
    pub(crate) title: String,
    pub(crate) verbose_title: String,
    pub(crate) commit_url: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct GithubResults {
    pub(crate) name: Option<String>,

    pub(crate) profile_url: Option<String>,
    pub(crate) commit_history: Vec<GithubCommitInfo>,
}

impl GithubResults {
    pub(crate) fn new() -> Self {
        Self {
            name: None,
            profile_url: None,
            commit_history: Vec::new(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct ProfileData;
