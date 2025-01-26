use super::identification_payment::PaymentOwner;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDealRequest {
    pub ext_key: String,
    pub amount: f64,
    pub payers: Vec<PaymentOwner>,
    pub recipients: Vec<DealRecipient>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DealRecipient {
    PaymentContract {
        number: u32,
        amount: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        purpose_nds: Option<f64>,
        account: String,
        bank_code: String,
        name: String,
        inn: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        kpp: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        document_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        purpose: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        code_purpose: Option<String>,
        identifier: String,
    },
    Commission {
        number: u32,
        amount: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        purpose: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        purpose_nds: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        purpose_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        document_number: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDealResponse {
    deal_id: String,
    // TODO
    compliance_check_payments: serde_json::Value
}
