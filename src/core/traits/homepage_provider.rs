use crate::generated::{PagedResults, Section, SectionItem};
use async_trait::async_trait;

#[async_trait]
pub trait HomepageProvider {
    fn get_homepage(&self) -> Option<Vec<Section>>;
    fn get_view_more_items(&self, section_id: &str) -> Result<PagedResults<SectionItem>, String>;
}
