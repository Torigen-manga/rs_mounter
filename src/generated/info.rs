use serde::{Deserialize, Serialize};
use super::{Capabilities, SourceDependencies};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SourceInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub icon: Option<String>,
    pub dependencies: Vec<SourceDependencies>,
    pub capabilities: Vec<Capabilities>,
}
