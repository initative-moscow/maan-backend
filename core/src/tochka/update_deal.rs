use serde::{Deserialize, Serialize};

use super::{create_deal::DealRecipient, identification_payment::PaymentOwner};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDealRequest {
    pub deal_id: String,
    pub deal_data: UpdateDealData
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDealData {
    pub amount: f64,
    pub payers: Vec<PaymentOwner>,
    pub recipients: Vec<DealRecipient>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDealResponse {
    deal_id: String,
    // TODO
    compliance_check_payments: serde_json::Value
}
