use crate::generated::{Chapter, ChapterEntry, HttpRequest, HttpResponse, Manga};

pub trait MangaProvider {
    fn get_manga_request(&self, manga_id: &str) -> HttpRequest;
    fn process_manga_res(&self, response: HttpResponse) -> Result<Manga, String>;

    fn get_chapters_request(&self, manga_id: &str) -> HttpRequest;
    fn process_chapters_res(&self, response: HttpResponse) -> Result<Vec<ChapterEntry>, String>;

    fn get_chapter_details_request(&self, manga_id: &str, chapter_id: &str) -> HttpRequest;
    fn process_chapter_details_res(&self, response: HttpResponse) -> Result<Chapter, String>;
}
