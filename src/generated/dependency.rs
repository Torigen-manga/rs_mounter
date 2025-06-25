use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SourceDependencies {
    pub name: String,
    pub version: Option<String>,
}