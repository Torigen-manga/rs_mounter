use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LoadingPattern {
    Continuous,
    Pagination,
    InfiniteScroll,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResultMetadata {
    pub loading_pattern: Option<LoadingPattern>,
    pub next_offset: Option<u32>,
    pub next_page: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PagedResults<T> {
    pub results: Vec<T>,

    pub total: u32,
    pub has_next_page: bool,
    pub has_previous_page: bool,

    pub current_page: u32,
    pub total_pages: u32,

    pub limit: u32,
    pub offset: Option<u32>,
    pub metadata: Option<ResultMetadata>,
}
