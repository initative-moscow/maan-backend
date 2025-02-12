use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDocumentRequest {
    pub document_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDocumentResponseIO {
    #[serde(rename = "document")]
    inner: GetDocumentResponse,
}

impl GetDocumentResponseIO {
    pub fn into_inner(self) -> GetDocumentResponse {
        self.inner
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDocumentResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub document_type: String,
    pub document_number: String,
    pub document_date: String,
    pub success_added: bool,
    pub success_added_desc: String,
    pub deal_id: Option<String>,
}
