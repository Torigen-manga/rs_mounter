use serde::{Deserialize, Serialize};

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