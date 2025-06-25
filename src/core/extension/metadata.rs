use crate::generated::{MetadataModel, MetadataSchemaMap};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait SourceMetadataProvider {
    fn get_metadata(&self) -> MetadataSchema;
}
