use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::generated::Tag;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TagOperator {
    And,
    Or,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SearchParamValue {
    String(String),
    Int(u32),
    Bool(bool),
    VectorString(Vec<String>),
    VectorInt(Vec<u32>),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SearchRequest {
    pub query: Option<String>,
    pub included_tags: Vec<Tag>,
    pub excluded_tags: Vec<Tag>,
    pub include_operator: Option<TagOperator>,
    pub exclude_operator: Option<TagOperator>,

    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,

    pub parameters: Option<HashMap<String, SearchParamValue>>,
}
