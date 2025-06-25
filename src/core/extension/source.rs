use super::{HomepageProvider, MangaProvider, SearchProvider, SourceMetadataProvider};
use crate::generated::SourceInfo;

pub trait SourceProvider:
    HomepageProvider + SearchProvider + MangaProvider + SourceMetadataProvider
{
    fn get_source_info(&self) -> SourceInfo;
}
