use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Chapter {
    pub id: String,
    pub title: String,
    pub volume: Option<String>,
    pub number: u32,
    pub pages: Vec<String>,
    pub scanlator: Option<String>,
    pub groups: Option<Vec<String>>,
    pub language: Option<String>,
    pub release_date: Option<String>,
    pub read_state: Option<ReadState>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ReadState {
    pub read: bool,
    pub timestamp: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChapterEntry {
    pub id: String,
    pub title: String,
    pub timestamp: String,
}

