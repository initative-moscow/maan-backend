use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVirtualAccountRequest {
    pub beneficiary_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBeneficiaryResponse {
    virtual_account: String,
}
