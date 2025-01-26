use serde::{Deserialize, Serialize};

use super::create_beneficiary::BeneficiaryData;


#[derive(Debug, Serialize, Deserialize)]
pub struct GetBeneficiaryRequest {
    pub beneficiary_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBeneficiaryResponse {
    pub beneficiary: GetBeneficiaryResponseBeneficiary,
    pub nominal_account: NominalAccount,
    pub last_contract_offer: serde_json::Value,
    pub permission: bool,
    pub permission_description: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBeneficiaryResponseBeneficiary {
    pub id: String,
    pub inn: String,
    pub is_active: bool,
    pub legal_type: String,
    pub ogrn: Option<String>,
    pub beneficiary_data: BeneficiaryData,
    pub created_at: String,
    pub updated_at: String,
    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NominalAccount {
    pub code: String,
    pub bic: String,
    pub is_added_to_ms: Option<bool>
}