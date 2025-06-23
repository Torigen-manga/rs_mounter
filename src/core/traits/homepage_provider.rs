use crate::generated::{Homepage, PagedResults, SectionEntry};
use async_trait::async_trait;

#[async_trait]
pub trait HomepageProvider {
    fn get_homepage(&self) -> Result<Homepage, String>;
    fn get_view_more_items(&self, section_id: &str) -> Result<PagedResults<SectionEntry>, String>;
}
