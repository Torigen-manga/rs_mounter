use super::{HomepageClient, MangaClient, SearchClient, SourceFieldsMetadata};
use crate::generated::SourceInfo;

pub trait SourceClient:
    HomepageClient + SearchClient + MangaClient + SourceFieldsMetadata
{
    fn get_source_info(&self) -> SourceInfo;
}
