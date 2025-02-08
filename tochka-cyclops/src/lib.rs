//! Tochka API

/*
TODO:
1. Introduce blocking API
2. Introduce a separate Error type

*/

pub mod tochka_io {
    //! Used to re-export Tochka API types.

    pub use super::create_beneficiary::*;
    pub use super::create_deal::*;
    pub use super::create_virtual_account::*;
    pub use super::execute_deal::*;
    pub use super::get_beneficiary::*;
    pub use super::get_deal::*;
    pub use super::get_document::*;
    pub use super::get_payment::*;
    pub use super::get_virtual_account::*;
    pub use super::identification_payment::*;
    pub use super::list_beneficiary::*;
    pub use super::list_deals::*;
    pub use super::list_payments::*;
    pub use super::list_virtual_account::*;
    pub use super::sbp_qrcode::*;
    pub use super::update_deal::*;
}

pub mod create_beneficiary;
pub mod create_deal;
pub mod create_virtual_account;
pub mod execute_deal;
pub mod get_beneficiary;
pub mod get_deal;
pub mod get_document;
pub mod get_payment;
pub mod get_virtual_account;
pub mod identification_payment;
pub mod list_beneficiary;
pub mod list_deals;
pub mod list_payments;
pub mod list_virtual_account;
pub mod sbp_qrcode;
pub mod update_deal;

mod client;
mod signer;
mod utils;

pub use client::{RequestSigner, TochkaCyclopsClient};
pub use signer::{DummySigner, RsaSigner, SignerError};
pub use utils::new_uuid_v4;

use serde::{Deserialize, Serialize};

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

impl<T> TochkaApiResponsePayload<T> {
    pub fn is_err(&self) -> bool {
        matches!(self, TochkaApiResponsePayload::Error { .. })
    }

    pub fn is_ok(&self) -> bool {
        matches!(self, TochkaApiResponsePayload::Result { .. })
    }
}

/// Exact tochka error type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TochkaError {
    pub code: u64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tochka_io::*;

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
                    is_branch: None,
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
                    meta: None,
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
        }

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
        let actual: TochkaApiResponse<CreateBeneficiaryResponse> =
            serde_json::from_value(json.clone()).expect("failed to parse json");
        assert_eq!(expected, actual);
        assert_eq!(
            serde_json::to_value(expected).expect("failed to serialize to string"),
            json
        );
    }
}
