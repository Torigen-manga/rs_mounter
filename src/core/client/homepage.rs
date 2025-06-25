use crate::generated::{Homepage, PagedResults, SectionEntry};
use async_trait::async_trait;

#[async_trait]
pub trait HomepageClient {
    async fn get_homepage(&self) -> Result<Homepage, String>;
    async fn get_view_more_items(&self, section_id: &str) -> Result<PagedResults<SectionEntry>, String>;
}
