use async_trait::async_trait;

use super::Buildrequest;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct PinterestResults;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Pinterest {
    base_url: String,
    search_query: String,
}

#[async_trait]
impl Buildrequest for Pinterest {
    type Item = PinterestResults;
    async fn search(&self) -> Result<Self::Item, anyhow::Error> {
        todo!()
    }
}
