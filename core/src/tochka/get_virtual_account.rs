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
    pub cash: u64,
    pub blocked_cash: u64,
    pub beneficiary_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVirtualAccountRequest {
    pub virtual_account: String,
}
