//! Tochka API

use serde::{Deserialize, Serialize};

pub mod create_beneficiary;
pub mod create_virtual_account;
pub mod get_virtual_account;
pub mod identification_payment;
pub mod list_beneficiary;
pub mod list_payments;
pub mod sbp_qrcode;
pub mod create_deal;

/// General tochka API JSON request type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TochkaApiRequest<T> {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: T,
}

/// General tochka API JSON response type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TochkaApiResponse<T> {
    pub jsonrpc: String,
    pub id: String,
    #[serde(flatten)]
    pub payload: TochkaApiResponsePayload<T>,
}

/// Tochka API response payload, which is either an "error" or a "result".
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum TochkaApiResponsePayload<T> {
    Error { error: TochkaError },
    Result { result: T },
}

// TODO: Include in error variant the meta data
/// Exact tochka error type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TochkaError {
    pub code: u64,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tochka::create_beneficiary::{
        BeneficiaryData, CreateBeneficiaryResponse, CreateBeneficiaryUlRequest,
    };

    #[test]
    fn test_serde_json_request() {
        let inn = "1234567890";
        let acc_code = "1234567890";
        let bic = "1234567890";
        let kpp = "123456789";

        let json = serde_json::json!({
            "jsonrpc": "2.0",
            "id": "123",
            "method": "create_beneficiary_ul",
            "params": {
                "inn": inn,
                "nominal_account_code": acc_code,
                "nominal_account_bic": bic,
                "beneficiary_data": {
                    "name": "Test",
                    "kpp": kpp
                }
            }
        });
        let expected = TochkaApiRequest {
            jsonrpc: "2.0".to_string(),
            id: "123".to_string(),
            method: "create_beneficiary_ul".to_string(),
            params: CreateBeneficiaryUlRequest {
                inn: inn.to_string(),
                nominal_account_code: acc_code.to_string(),
                nominal_account_bic: bic.to_string(),
                beneficiary_data: BeneficiaryData {
                    name: "Test".to_string(),
                    kpp: kpp.to_string(),
                    ogrn: None,
                },
            },
        };

        let actual: TochkaApiRequest<CreateBeneficiaryUlRequest> =
            serde_json::from_value(json.clone()).expect("failed to parse json");
        assert_eq!(actual, expected);
        assert_eq!(
            serde_json::to_value(expected).expect("failed to serialize to string"),
            json
        );
    }

    #[test]
    fn test_json_serde_response() {
        // Error response
        let json = serde_json::json!({
            "jsonrpc": "2.0",
            "id": "123",
            "error": { "code": 1, "message": "message" }
        });

        let expected = TochkaApiResponse {
            jsonrpc: "2.0".to_string(),
            id: "123".to_string(),
            payload: TochkaApiResponsePayload::Error {
                error: TochkaError {
                    code: 1,
                    message: "message".to_string(),
                },
            },
        };

        let actual: TochkaApiResponse<()> =
            serde_json::from_value(json.clone()).expect("failed to parse json");
        assert_eq!(actual, expected);
        assert_eq!(
            serde_json::to_value(expected).expect("failed to serialize to string"),
            json
        );

        // Result response
        let json = serde_json::json!({
            "jsonrpc": "2.0",
            "id": "123",
            "result": { "value": 42 }
        });

        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
        struct Value {
            value: u64,
        };

        let expected = TochkaApiResponse {
            jsonrpc: "2.0".to_string(),
            id: "123".to_string(),
            payload: TochkaApiResponsePayload::Result {
                result: Value { value: 42 },
            },
        };

        let actual: TochkaApiResponse<Value> =
            serde_json::from_value(json.clone()).expect("failed to parse json");
        assert_eq!(actual, expected);
        assert_eq!(
            serde_json::to_value(expected).expect("failed to serialize to string"),
            json
        );

        // Real type response
        let json = serde_json::json!({
            "jsonrpc": "2.0",
            "result": {
                "beneficiary": {
                    "inn": "7925930371",
                    "id": "4242",
                    "nominal_account_code": "000000000000000000000",
                    "nominal_account_bic": "0000000000",
                }
            },
            "id": "42"
        });
        let expected = TochkaApiResponse {
            jsonrpc: "2.0".to_string(),
            id: "42".to_string(),
            payload: TochkaApiResponsePayload::Result {
                result: CreateBeneficiaryResponse::Beneficiary {
                    inn: "7925930371".to_string(),
                    id: "4242".to_string(),
                    nominal_account_code: "000000000000000000000".to_string(),
                    nominal_account_bic: "0000000000".to_string(),
                },
            },
        };
        println!("{:#?}", serde_json::to_value(&expected).unwrap());
        let actual: TochkaApiResponse<CreateBeneficiaryResponse> =
            serde_json::from_value(json.clone()).expect("failed to parse json");
        assert_eq!(expected, actual);
        assert_eq!(
            serde_json::to_value(expected).expect("failed to serialize to string"),
            json
        );
    }
}
