use async_trait::async_trait;

use super::Buildrequest;
use crate::types::{PinterestResults, ProfileData};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Pinterest {
    base_url: String,
    search_query: String,
}

#[async_trait]
impl Buildrequest for Pinterest {
    async fn search(&self, result: &mut ProfileData) -> Result<(), anyhow::Error> {
        todo!()
    }
}
