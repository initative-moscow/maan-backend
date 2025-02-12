use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDealRequest {
    pub deal_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDealResponse {
    pub deal: Deal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deal {
    pub id: String,
    pub ext_key: String,
    pub amount: f64,
    pub status: String,
    pub payers: Vec<Payer>,
    pub recipients: Vec<Recipient>,
    // TODO chrono::NaiveDateTime
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payer {
    pub virtual_account: String,
    pub amount: f64,
    pub documents: Vec<Document>,
    pub executed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    #[serde(rename = "type")]
    pub document_type: String,
    pub success_added: bool,
    pub success_added_desc: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipient {
    pub number: u32,
    pub amount: f64,
    pub executed: Option<bool>,
    #[serde(rename = "type")]
    pub recipient_type: String,
    pub requisites: Requisites,
    pub payment: Payment,
    pub error_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Requisites {
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    pub id: String,
    // TODO - to enum
    pub status: String,
    // TODO
    pub meta: Option<serde_json::Value>,
}
