pub trait WasmExtension {
    fn get_source_info(&self) -> String;
    fn get_metadata(&self) -> String;

    fn get_homepage_request(&self) -> String;
    fn process_homepage_res(&self, responses_json: &str) -> String;
    fn get_viewmore_request(&self, section_id: &str, page: u32) -> String;
    fn process_viewmore_res(&self, response_json: &str) -> String;

    fn get_search_request(&self, query_json: &str) -> String;
    fn process_search_res(&self, response_json: &str) -> String;
    fn get_search_tags_request(&self) -> String;
    fn process_search_tags_res(&self, response_json: &str) -> String;

    fn get_manga_request(&self, manga_id: &str) -> String;
    fn process_manga_res(&self, response_json: &str) -> String;
    fn get_chapters_request(&self, manga_id: &str) -> String;
    fn process_chapters_res(&self, response_json: &str) -> String;
    fn get_chapter_details_request(&self, manga_id: &str, chapter_id: &str) -> String;
    fn process_chapter_details_res(&self, responses_json: &str) -> String;
}

pub enum ExtensionMethods {
    GetSourceInfo,
    GetMetadata,
    GetHomepageRequest,
    ProcessHomepageRes,
    GetViewmoreRequest,
    ProcessViewmoreRes,
    GetSearchRequest,
    ProcessSearchRes,
    GetSearchTagsRequest,
    ProcessSearchTagsRes,
    GetMangaRequest,
    ProcessMangaRes,
    GetChaptersRequest,
    ProcessChaptersRes,
    GetChapterDetailsRequest,
    ProcessChapterDetailsRes,
}

impl ExtensionMethods {
    pub fn from_str(method: &str) -> Option<Self> {
        match method {
            "get_source_info" => Some(Self::GetSourceInfo),
            "get_metadata" => Some(Self::GetMetadata),
            "get_homepage_request" => Some(Self::GetHomepageRequest),
            "process_homepage_res" => Some(Self::ProcessHomepageRes),
            "get_viewmore_request" => Some(Self::GetViewmoreRequest),
            "process_viewmore_res" => Some(Self::ProcessViewmoreRes),
            "get_search_request" => Some(Self::GetSearchRequest),
            "process_search_res" => Some(Self::ProcessSearchRes),
            "get_search_tags_request" => Some(Self::GetSearchTagsRequest),
            "process_search_tags_res" => Some(Self::ProcessSearchTagsRes),
            "get_manga_request" => Some(Self::GetMangaRequest),
            "process_manga_res" => Some(Self::ProcessMangaRes),
            "get_chapters_request" => Some(Self::GetChaptersRequest),
            "process_chapters_res" => Some(Self::ProcessChaptersRes),
            "get_chapter_details_request" => Some(Self::GetChapterDetailsRequest),
            "process_chapter_details_res" => Some(Self::ProcessChapterDetailsRes),
            _ => None,
        }
    }
}
