use super::MangaEntry;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SectionType {
    SingleRowNormal,
    SingleRowLarge,
    DoubleRow,
    DoubleRowLarge,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TextEntry {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
pub enum SectionEntry {
    Manga(MangaEntry),
    Text(TextEntry),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Section {
    pub id: String,
    pub title: String,
    pub items: Vec<SectionEntry>,
    pub section_type: SectionType,
    pub contain_more: bool,
}

pub type Homepage = Vec<Section>;
