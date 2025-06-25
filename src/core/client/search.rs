use crate::generated::{MangaEntry, PagedResults, SearchRequest, Tag};
use async_trait::async_trait;

#[async_trait]
pub trait SearchClient {
    async fn get_search_results(
        &self,
        query: SearchRequest,
    ) -> Result<PagedResults<MangaEntry>, String>;
    async fn get_search_tags(&self) -> Result<Vec<Tag>, String> {
        Ok(vec![])
    }
}
