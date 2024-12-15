//! Create beneficiary request.

use serde::{Deserialize, Serialize};

/// Create beneficiary response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "beneficiary")]
pub struct CreateBeneficiaryResponse {
    // TODO: Create Inn type with validation
    pub inn: String,
    pub id: String,
    pub nominal_account_code: String,
    pub nominal_account_bic: String,
}


/// Create beneficiary ul request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateBeneficiaryUlRequest {
    pub inn: String,
    pub nominal_account_code: String,
    pub nominal_account_bic: String,
    pub beneficiary_data: BeneficiaryData,
}

/// Beneficiary data for creating beneficiary request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BeneficiaryData {
    pub name: String,
    pub kpp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ogrn: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_serde() {
        let inn = "1234567890";
        let acc_code = "1234567890";
        let bic = "1234567890";
        let name = "Test";
        let kpp = "123456789";

        let expected_request = CreateBeneficiaryUlRequest {
            inn: inn.to_string(),
            nominal_account_code: acc_code.to_string(),
            nominal_account_bic: bic.to_string(),
            beneficiary_data: BeneficiaryData {
                name: name.to_string(),
                kpp: kpp.to_string(),
                ogrn: None,
            },
        };
        let json_request = serde_json::json!({
            "inn": inn,
            "nominal_account_code": acc_code,
            "nominal_account_bic": bic,
            "beneficiary_data": {
                "name": name,
                "kpp": kpp,
            },
        });
        let actual_request: CreateBeneficiaryUlRequest = serde_json::from_value(json_request.clone()).expect("failed to parse json");
        assert_eq!(expected_request, actual_request);
        
        let actual_request = serde_json::to_value(expected_request.clone()).expect("failed to serialize json");
        assert_eq!(json_request, actual_request);
        
        
        let json_response = serde_json::json!({
            "inn": inn,
            "id": "1234567890",
            "nominal_account_code": acc_code,
            "nominal_account_bic": bic,
        });
    }
}