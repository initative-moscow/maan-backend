use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ListVirtualAccountRequest {
    pub filters: ListBeneficiaryFilters
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListBeneficiaryFilters {
    pub beneficiary: BeneficiaryFilter 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeneficiaryFilter {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub beneficiary_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inn: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListVirtualAccountResponse {
    virtual_accounts: Vec<String>,
}