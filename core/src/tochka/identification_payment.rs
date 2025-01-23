use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentificationPaymentRequest {
    pub payment_id: String,
    pub owners: Vec<PaymentOwner>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentOwner {
    pub virtual_account: String,
    pub amount: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentificationPaymentResponse {
    pub virtual_accounts: Vec<VirtualAccountsResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualAccountsResponse {
    pub code: String,
    pub cash: u32,
}
