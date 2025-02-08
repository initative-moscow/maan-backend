use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVirtualAccountResponseIO {
    #[serde(rename = "virtual_account")]
    inner: GetVirtualAccountResponse,
}

impl GetVirtualAccountResponseIO {
    pub fn into_inner(self) -> GetVirtualAccountResponse {
        self.inner
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVirtualAccountResponse {
    #[serde(rename = "code")]
    pub virtual_account_id: String,
    #[serde(rename = "type")]
    pub virtual_account_type: String,
    // cash - free money
    pub cash: f64,
    // blocked - после создания сделки
    pub blocked_cash: f64,
    pub beneficiary_id: String,
    pub beneficiary_inn: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVirtualAccountRequest {
    pub virtual_account: String,
}
