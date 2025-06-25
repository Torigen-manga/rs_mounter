use super::{HomepageProvider, MangaProvider, SearchProvider, SourceFieldsMetadata};
use crate::generated::SourceInfo;

pub trait SourceProvider:
    HomepageProvider + SearchProvider + MangaProvider + SourceFieldsMetadata
{
    fn get_source_info(&self) -> SourceInfo;
}
