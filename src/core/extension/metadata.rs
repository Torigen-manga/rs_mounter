use crate::generated::{MetadataModel, MetadataSchemaMap};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MetadataSchema {
    pub view_more: HashMap<String, MetadataSchemaMap>,
    pub search: HashMap<String, MetadataModel>,
}

pub trait SourceMetadataProvider {
    fn get_metadata(&self) -> MetadataSchema;
}
