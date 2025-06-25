use serde::{Deserialize, Serialize};
use super::{Capabilities, SourceDependencies};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SourceInfo {
    pub id: String,
    pub name: String,
    pub icon_url: String,
    pub base_url: String,
    pub version: String,
    pub dependencies: Vec<SourceDependencies>,
    pub capabilities: Vec<Capabilities>,
}
