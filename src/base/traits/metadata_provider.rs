use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SelectModel {
    Default(String),
    Multiple(Vec<String>),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum MetadataValue {
    Number(u32),
    String(String),
    Bool(bool),
    Select(SelectModel),
    Null,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum MetadataType {
    Number,
    String,
    Bool,
    Select,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MetadataModel {
    pub metadata_type: MetadataType,
    pub required: bool,
    pub default_value: Option<MetadataValue>,
    pub options: Option<Vec<String>>,
    pub description: Option<String>,
    pub min: Option<u32>,
    pub max: Option<u32>,
    pub pattern: Option<String>,
}

pub type MetadataSchemaMap = HashMap<String, MetadataModel>;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MetadataSchema {
    pub view_more: HashMap<String, MetadataSchemaMap>,
    pub search: HashMap<String, MetadataModel>,
}

pub trait SourceFieldsMetadata {
    fn get_metadata(&self) -> MetadataSchema;
}
