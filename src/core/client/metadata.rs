use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::generated::{MetadataModel, MetadataSchemaMap};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MetadataSchema {
    pub view_more: HashMap<String, MetadataSchemaMap>,
    pub search: HashMap<String, MetadataModel>,
}

pub trait SourceFieldsMetadata {
    fn get_metadata(&self) -> MetadataSchema;
}
