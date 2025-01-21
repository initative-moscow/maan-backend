use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListBeneficiaryRequest {
    pub filters: ListBeneficiaryFilters,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListBeneficiaryFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nominal_account_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nominal_account_bic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_type: Option<LegalType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListBeneficiaryResponse{
    pub beneficiaries: Vec<ListBeneficiaryData>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListBeneficiaryData{
    pub id: String,
    pub inn: String,
    pub nominal_account_code: String,
    pub nominal_account_bic: String,
    pub is_active: bool,
    pub legal_type: LegalType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalType {
    F,
    I,
    J
}

impl Display for LegalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LegalType::F => write!(f, "F"),
            LegalType::I => write!(f, "I"),
            LegalType::J => write!(f, "J"),
        }
    }
}