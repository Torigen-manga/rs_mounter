use crate::generated::{Homepage, HttpRequest, HttpResponse, PagedResults, SectionEntry};

pub trait HomepageProvider {
    fn get_homepage_request(&self) -> Vec<HttpRequest>;
    fn process_homepage_res(&self, responses: Vec<HttpResponse>) -> Result<Homepage, String>;

    fn get_viewmore_request(&self, section_id: &str, page: u32) -> HttpRequest;
    fn process_viewmore_res(
        &self,
        response: HttpResponse,
    ) -> Result<PagedResults<SectionEntry>, String>;
}
