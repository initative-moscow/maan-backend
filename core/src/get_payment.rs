use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPaymentRequest {
    pub payment_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPaymentResponseIO {
    #[serde(rename = "payment")]
    pub inner: GetPaymentResponse,
}

impl GetPaymentResponseIO {
    pub fn into_inner(self) -> GetPaymentResponse {
        self.inner
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPaymentResponse {
    pub id: String,
    pub amount: f64,
    pub document_number: String,
    pub deal_id: Option<String>,
    pub document_date: String,
    // TODO enum
    pub status: String,
    #[serde(rename = "type")]
    pub payment_type: String,
    pub purpose: String,
    pub created_at: String,
    pub updated_at: String,
    pub incoming: bool,
    pub identify: bool,
    pub qrcode_id: Option<String>,
    pub payer: PaymentParticipant,
    pub recipient: PaymentParticipant,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentParticipant {
    pub account: String,
    pub bank_code: String,
    pub bank_name: String,
    pub name: String,
    pub tax_code: String,
    pub tax_reason_code: Option<String>,
    pub bank_correspondent_account: Option<String>,
}
