use super::{HomepageClient, MangaClient, SearchClient, SourceFieldsMetadata};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SourceDependencies {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Capabilities {
    #[serde(rename = "supports_homepage")]
    SupportsHomepage,
    #[serde(rename = "supports_search")]
    SupportsSearch,
    #[serde(rename = "supports_view_more")]
    SupportsViewMore,
    #[serde(rename = "supports_pagination")]
    SupportsPagination,
    #[serde(rename = "supports_include_tags")]
    SupportsIncludeTags,
    #[serde(rename = "supports_exclude_tags")]
    SupportsExcludeTags,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SourceInfo {
    pub id: String,
    pub name: String,
    pub icon_url: String,
    pub base_url: String,
    pub version: Option<String>,
    pub dependencies: Option<Vec<SourceDependencies>>,
    pub capabilities: Vec<Capabilities>,
}

pub trait SourceClient:
    HomepageClient + SearchClient + MangaClient + SourceFieldsMetadata
{
    fn get_source_info(&self) -> SourceInfo;
}
