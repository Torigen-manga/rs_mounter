
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SectionType {
    SingleRowNormal,
    SingleRowLarge,
    DoubleRow,
    DoubleRowLarge,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SectionItem {
    pub id: String,
    pub title: String,
    pub image: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Section {
    pub id: String,
    pub title: String,
    pub items: Vec<SectionItem>,
    pub section_type: SectionType,
    pub contain_more: bool,
}
