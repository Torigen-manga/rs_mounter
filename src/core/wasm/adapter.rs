use super::structure::WasmExtension;
use crate::{core::extension::*, generated::*};
use serde_json::{from_str, to_string};

pub struct WasmExtensionWrapper<T: SourceProvider> {
    pub inner: T,
}

impl<T: SourceProvider> WasmExtensionWrapper<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: SourceProvider> WasmExtension for WasmExtensionWrapper<T> {
    fn get_source_info(&self) -> String {
        to_string(&self.inner.get_source_info()).unwrap_or_default()
    }

    fn get_metadata(&self) -> String {
        to_string(&self.inner.get_metadata()).unwrap_or_default()
    }

    fn get_homepage_request(&self) -> String {
        to_string(&self.inner.get_homepage_request()).unwrap_or_default()
    }

    fn process_homepage_res(&self, responses_json: &str) -> String {
        let responses: Vec<HttpResponse> = match from_str(responses_json) {
            Ok(r) => r,
            Err(e) => {
                return to_string(&Result::<Homepage, String>::Err(e.to_string()))
                    .unwrap_or_default();
            }
        };

        let result = self.inner.process_homepage_res(responses);
        to_string(&result).unwrap_or_default()
    }

    fn get_viewmore_request(&self, section_id: &str, page: u32) -> String {
        to_string(&self.inner.get_viewmore_request(section_id, page)).unwrap_or_default()
    }

    fn process_viewmore_res(&self, response_json: &str) -> String {
        let response: HttpResponse = match from_str(response_json) {
            Ok(r) => r,
            Err(e) => {
                return to_string(&Result::<PagedResults<SectionEntry>, String>::Err(
                    e.to_string(),
                ))
                .unwrap_or_default();
            }
        };

        let result = self.inner.process_viewmore_res(response);
        to_string(&result).unwrap_or_default()
    }

    fn get_search_request(&self, query_json: &str) -> String {
        let query: SearchRequest = match from_str(query_json) {
            Ok(q) => q,
            Err(e) => return to_string(&format!("Error: {}", e)).unwrap_or_default(),
        };

        to_string(&self.inner.get_search_request(&query)).unwrap_or_default()
    }

    fn process_search_res(&self, response_json: &str) -> String {
        let response: HttpResponse = match from_str(response_json) {
            Ok(r) => r,
            Err(e) => {
                return to_string(&Result::<PagedResults<MangaEntry>, String>::Err(
                    e.to_string(),
                ))
                .unwrap_or_default();
            }
        };

        let result = self.inner.process_search_res(response);
        to_string(&result).unwrap_or_default()
    }

    fn get_search_tags_request(&self) -> String {
        to_string(&self.inner.get_search_tags_request()).unwrap_or_default()
    }

    fn process_search_tags_res(&self, response_json: &str) -> String {
        let response: HttpResponse = match from_str(response_json) {
            Ok(r) => r,
            Err(e) => {
                return to_string(&Result::<Vec<Tag>, String>::Err(e.to_string()))
                    .unwrap_or_default();
            }
        };

        let result = self.inner.process_search_tags_res(response);
        to_string(&result).unwrap_or_default()
    }

    fn get_manga_request(&self, manga_id: &str) -> String {
        to_string(&self.inner.get_manga_request(manga_id)).unwrap_or_default()
    }

    fn process_manga_res(&self, response_json: &str) -> String {
        let response: HttpResponse = match from_str(response_json) {
            Ok(r) => r,
            Err(e) => {
                return to_string(&Result::<Manga, String>::Err(e.to_string())).unwrap_or_default();
            }
        };

        let result = self.inner.process_manga_res(response);
        to_string(&result).unwrap_or_default()
    }

    fn get_chapters_request(&self, manga_id: &str) -> String {
        to_string(&self.inner.get_chapters_request(manga_id)).unwrap_or_default()
    }

    fn process_chapters_res(&self, response_json: &str) -> String {
        let response: HttpResponse = match from_str(response_json) {
            Ok(r) => r,
            Err(e) => {
                return to_string(&Result::<Vec<ChapterEntry>, String>::Err(e.to_string()))
                    .unwrap_or_default();
            }
        };

        let result = self.inner.process_chapters_res(response);
        to_string(&result).unwrap_or_default()
    }

    fn get_chapter_details_request(&self, manga_id: &str, chapter_id: &str) -> String {
        to_string(&self.inner.get_chapter_details_request(manga_id, chapter_id)).unwrap_or_default()
    }

    fn process_chapter_details_res(&self, responses_json: &str) -> String {
        let responses: Vec<HttpResponse> = match from_str(responses_json) {
            Ok(r) => r,
            Err(e) => {
                return to_string(&Result::<Chapter, String>::Err(e.to_string()))
                    .unwrap_or_default();
            }
        };

        let result = self.inner.process_chapter_details_res(responses);
        to_string(&result).unwrap_or_default()
    }
}

pub trait WasmExtensionProvider {
    fn new() -> Self;
}

#[macro_export]
macro_rules! export_wasm_extension {
    ($extension_type:ty) => {
        use std::sync::OnceLock;
        use torigen_core::WasmExtension;
        use torigen_wasm_helper::{WasmExtensionProvider, WasmExtensionWrapper};

        static EXTENSION: OnceLock<WasmExtensionWrapper<$extension_type>> = OnceLock::new();

        fn get_extension() -> &'static WasmExtensionWrapper<$extension_type> {
            EXTENSION.get_or_init(|| WasmExtensionWrapper::new(<$extension_type>::new()))
        }

        #[no_mangle]
        pub unsafe extern "C" fn extension_call(
            method_name_ptr: *const u8,
            method_name_len: usize,
            args_ptr: *const u8,
            args_len: usize,
            result_ptr: *mut u8,
            result_len: *mut usize,
        ) -> i32 {
            let method_name = match unsafe {
                std::str::from_utf8(std::slice::from_raw_parts(method_name_ptr, method_name_len))
            } {
                Ok(s) => s,
                Err(_) => return -1,
            };

            let args = if args_len > 0 {
                match unsafe { std::str::from_utf8(std::slice::from_raw_parts(args_ptr, args_len)) }
                {
                    Ok(s) => s,
                    Err(_) => return -1,
                }
            } else {
                ""
            };

            let method = match ExtensionMethods::from_str(method_name) {
                Some(m) => m,
                None => return -1,
            };

            let result = match method {
                ExtensionMethods::GetSourceInfo => get_extension().get_source_info(),
                ExtensionMethods::GetMetadata => get_extension().get_metadata(),
                ExtensionMethods::GetHomepageRequest => get_extension().get_homepage_request(),
                ExtensionMethods::ProcessHomepageRes => get_extension().process_homepage_res(args),
                ExtensionMethods::GetViewmoreRequest => {
                    let parsed: Result<serde_json::Value, _> = serde_json::from_str(args);
                    match parsed {
                        Ok(json) => {
                            let section_id = json["section_id"].as_str().unwrap_or("");
                            let page = json["page"].as_u64().unwrap_or(1) as u32;
                            get_extension().get_viewmore_request(section_id, page)
                        }
                        Err(_) => return -1,
                    }
                }
                ExtensionMethods::ProcessViewmoreRes => get_extension().process_viewmore_res(args),
                ExtensionMethods::GetSearchRequest => get_extension().get_search_request(args),
                ExtensionMethods::ProcessSearchRes => get_extension().process_search_res(args),
                ExtensionMethods::GetSearchTagsRequest => get_extension().get_search_tags_request(),
                ExtensionMethods::ProcessSearchTagsRes => {
                    get_extension().process_search_tags_res(args)
                }
                ExtensionMethods::GetMangaRequest => get_extension().get_manga_request(args),
                ExtensionMethods::ProcessMangaRes => get_extension().process_manga_res(args),
                ExtensionMethods::GetChaptersRequest => get_extension().get_chapters_request(args),
                ExtensionMethods::ProcessChaptersRes => get_extension().process_chapters_res(args),
                ExtensionMethods::GetChapterDetailsRequest => {
                    let parsed: Result<serde_json::Value, _> = serde_json::from_str(args);
                    match parsed {
                        Ok(json) => {
                            let manga_id = json["manga_id"].as_str().unwrap_or("");
                            let chapter_id = json["chapter_id"].as_str().unwrap_or("");
                            get_extension().get_chapter_details_request(manga_id, chapter_id)
                        }
                        Err(_) => return -1,
                    }
                }
                ExtensionMethods::ProcessChapterDetailsRes => {
                    get_extension().process_chapter_details_res(args)
                }
            };

            let result_bytes = result.as_bytes();
            if result_bytes.len() <= unsafe { *result_len } {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        result_bytes.as_ptr(),
                        result_ptr,
                        result_bytes.len(),
                    );
                    *result_len = result_bytes.len();
                }
                0
            } else {
                unsafe {
                    *result_len = result_bytes.len();
                }
                1
            }
        }
    };
}
