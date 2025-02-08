use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenerateSbpQrCodeRequest {
    pub amount: u32,
    pub purpose: String,
    pub nominal_account_code: String,
    pub nominal_account_bic: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenerateSbpQrCodeResponseIO {
    #[serde(rename = "qrcode")]
    inner: GenerateSbpQrCodeResponse,
}

impl GenerateSbpQrCodeResponseIO {
    pub fn into_inner(self) -> GenerateSbpQrCodeResponse {
        self.inner
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenerateSbpQrCodeResponse {
    pub id: String,
    pub url: String,
    pub image: QrCodeImage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QrCodeImage {
    pub media_type: String,
    pub content_base64: String,
    pub width: u32,
    pub height: u32,
}
