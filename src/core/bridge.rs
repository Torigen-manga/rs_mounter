use crate::generated::*;

pub trait HomepageProvider {
    fn get_homepage_request(&self) -> Vec<HttpRequest>;
    fn process_homepage_res(&self, responses: Vec<HttpResponse>) -> Result<Homepage, String>;

    fn get_viewmore_request(&self, section_id: &str, page: u32) -> HttpRequest;
    fn process_viewmore_res(
        &self,
        response: HttpResponse,
    ) -> Result<PagedResults<SectionEntry>, String>;
}

pub trait MangaProvider {
    fn get_manga_request(&self, manga_id: &str) -> HttpRequest;
    fn process_manga_res(&self, response: HttpResponse) -> Result<Manga, String>;

    fn get_chapters_request(&self, manga_id: &str) -> HttpRequest;
    fn process_chapters_res(&self, response: HttpResponse) -> Result<Vec<ChapterEntry>, String>;

    fn get_chapter_details_request(&self, manga_id: &str, chapter_id: &str) -> Vec<HttpRequest>;
    fn process_chapter_details_res(&self, responses: Vec<HttpResponse>) -> Result<Chapter, String>;
}

pub trait SourceMetadataProvider {
    fn get_metadata(&self) -> MetadataSchema;
}

pub trait SearchProvider {
    fn get_search_request(&self, query: &SearchRequest) -> HttpRequest;
    fn process_search_res(
        &self,
        response: HttpResponse,
    ) -> Result<PagedResults<MangaEntry>, String>;

    fn get_search_tags_request(&self) -> HttpRequest;
    fn process_search_tags_res(&self, response: HttpResponse) -> Result<Vec<Tag>, String>;
}

pub trait SourceProvider:
    HomepageProvider + SearchProvider + MangaProvider + SourceMetadataProvider
{
    fn get_source_info(&self) -> SourceInfo;
}

// WASM Export Interface
// This is what extensions need to implement for WASM exports
pub trait WasmExtension {
    // Source Info
    fn get_source_info(&self) -> String; // JSON serialized SourceInfo
    fn get_metadata(&self) -> String; // JSON serialized MetadataSchema

    // Homepage
    fn get_homepage_request(&self) -> String; // JSON serialized Vec<HttpRequest>
    fn process_homepage_res(&self, responses_json: &str) -> String; // JSON serialized Result<Homepage, String>
    fn get_viewmore_request(&self, section_id: &str, page: u32) -> String; // JSON serialized HttpRequest
    fn process_viewmore_res(&self, response_json: &str) -> String; // JSON serialized Result<PagedResults<SectionEntry>, String>

    // Search
    fn get_search_request(&self, query_json: &str) -> String; // JSON serialized HttpRequest
    fn process_search_res(&self, response_json: &str) -> String; // JSON serialized Result<PagedResults<MangaEntry>, String>
    fn get_search_tags_request(&self) -> String; // JSON serialized HttpRequest
    fn process_search_tags_res(&self, response_json: &str) -> String; // JSON serialized Result<Vec<Tag>, String>

    // Manga
    fn get_manga_request(&self, manga_id: &str) -> String; // JSON serialized HttpRequest
    fn process_manga_res(&self, response_json: &str) -> String; // JSON serialized Result<Manga, String>
    fn get_chapters_request(&self, manga_id: &str) -> String; // JSON serialized HttpRequest
    fn process_chapters_res(&self, response_json: &str) -> String; // JSON serialized Result<Vec<ChapterEntry>, String>
    fn get_chapter_details_request(&self, manga_id: &str, chapter_id: &str) -> String; // JSON serialized Vec<HttpRequest>
    fn process_chapter_details_res(&self, responses_json: &str) -> String; // JSON serialized Result<Chapter, String>
}
