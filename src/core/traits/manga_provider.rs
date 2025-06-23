use crate::generated::{Chapter, ChapterEntry, Manga};
use async_trait::async_trait;

#[async_trait]
pub trait MangaProvider {
    fn get_manga_details(&self, manga_id: &str) -> Result<Manga, String>;
    fn get_chapters(&self, manga_id: &str) -> Result<Vec<ChapterEntry>, String>;
    fn get_chapter_details(&self, manga_id: &str, chapter_id: &str) -> Result<Chapter, String>;
}
