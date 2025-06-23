use serde::{Deserialize, Serialize};
use super::Tag;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Status {
    Ongoing,
    Completed,
    Hiatus,
    Cancelled,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Manga {
    pub image: String,
    pub title: String,
    pub description: String,
    pub artists: Vec<String>,
    pub authors: Vec<String>,
    pub tags: Vec<Tag>,
    pub status: Status,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MangaEntry {
    pub id: String,
    pub title: String,
    pub image: String,
}
