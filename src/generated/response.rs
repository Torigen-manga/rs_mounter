use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct HttpResponse {
    pub id: String,
    pub status: u16,
    pub body: Option<String>,
    pub error: Option<String>,
}
