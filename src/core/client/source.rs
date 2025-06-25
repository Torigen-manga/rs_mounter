use super::{HomepageClient, MangaClient, SearchClient, SourceMetadataClient};
use crate::generated::SourceInfo;

pub trait SourceClient:
    HomepageClient + SearchClient + MangaClient + SourceMetadataClient
{
    fn get_source_info(&self) -> SourceInfo;
}
