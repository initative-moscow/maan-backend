use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListPaymentsRequest {
    pub filters: ListPaymentsFilters,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListPaymentsFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c2b_qr_code_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incoming: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    // TODO bic must be in if account is set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>
}

// todo add meta
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListPaymentsResponse {
    pub payments: Vec<String>,
}
