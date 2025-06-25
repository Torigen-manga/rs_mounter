use crate::generated::{HttpRequest, HttpResponse, MangaEntry, PagedResults, SearchRequest, Tag};

pub trait SearchProvider {
    fn get_search_request(&self, query: &SearchRequest) -> HttpRequest;
    fn process_search_res(
        &self,
        response: HttpResponse,
    ) -> Result<PagedResults<MangaEntry>, String>;

    fn get_search_tags_request(&self) -> HttpRequest;
    fn process_search_tags_res(&self, response: HttpResponse) -> Result<Vec<Tag>, String>;
}
