use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVirtualAccountRequest {
    pub beneficiary_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_account_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVirtualAccountResponse {
    pub virtual_account: String,
}
