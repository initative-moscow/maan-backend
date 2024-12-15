use crate::{utils, Signer};
use anyhow::Result;
use reqwest::{
    blocking::{Client, Response},
    Method,
};

const PLATFORM_ID: &str = "maan";
const CERT_THUMBPRINT: &str = "";

pub(crate) struct MaanClient {
    sign_system: String,
    sign_thumbprint: String,
}

impl MaanClient {
    const TOCHKA_API_ENDPOINT: &'static str = "https://pre.tochka.com/api/v1/cyclops/v2/jsonrpc";

    pub(crate) fn new(sign_system: String, sign_thumbprint: String) -> Self {
        MaanClient {
            sign_system,
            sign_thumbprint,
        }
    }

    pub(crate) fn send_request(
        &self,
        signer: &Signer,
        body: impl AsRef<[u8]> + Clone,
    ) -> Result<Response> {
        let body = body.as_ref().to_vec();
        let b64_body_signature = {
            let signed_body = signer.sign_raw_data(&body)?;
            utils::base64_encode(signed_body)
        };

        let client = Client::new();
        let request = client
            .request(Method::POST, Self::TOCHKA_API_ENDPOINT)
            .header("sign-system", &self.sign_system)
            .header("sign-thumbprint", &self.sign_thumbprint)
            .header("sign-data", &b64_body_signature)
            .header("Content-Type", "application/json")
            .body(body);

        log::trace!("Sending request: {:#?}", request);

        request.send().map_err(Into::into)
    }

    pub(crate) fn send_request_tenders(
        &self,
        signer: &Signer,
        body: impl AsRef<[u8]> + Clone,
    ) -> Result<Response> {
        let body = body.as_ref().to_vec();
        let b64_body_signature = {
            let signed_body = signer.sign_raw_data(&body)?;
            utils::base64_encode(signed_body)
        };

        let client = Client::new();
        let request = client
            .request(Method::POST, "https://pre.tochka.com/api/v1/tender-helpers/jsonrpc")
            .header("sign-system", &self.sign_system)
            .header("sign-thumbprint", &self.sign_thumbprint)
            .header("sign-data", &b64_body_signature)
            .header("Content-Type", "application/json")
            .body(body);

        log::trace!("Sending request: {:#?}", request);

        request.send().map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, f32::consts::LOG10_2};

    use super::*;
    use serde::Deserialize;

/*
Maan:
Номинальный счёт: 40702810620000088278
БИК: 044525104
*/

    #[test]
    fn test_send_echo() {
        let _ = env_logger::Builder::from_default_env()
            .format_module_path(false)
            .format_level(true)
            .try_init();

        let signer = Signer::new().expect("failed signer creation");
        let maan_client = MaanClient::new(PLATFORM_ID.to_string(), CERT_THUMBPRINT.to_string());
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": utils::new_uuid_v4().to_string(),
            "method": "echo",
            "params": {"text": "Hello World!"},
        });
        let request_bytes = serde_json::to_vec(&request).expect("failed serializing json value");

        maan_client.send_request(&signer, request_bytes).expect("failed");
    }

    #[test]
    fn test_create_beneficiary() {

//         #[derive(Debug, Deserialize)]
//         struct CreateBeneficiaryResponse {
//             jsonrpc: String,
//             id: String,
//             result: serde_json::Value,
//         }

        let _ = env_logger::Builder::from_default_env()
            .format_module_path(false)
            .format_level(true)
            .try_init();
        let signer = Signer::new().expect("failed signer creation");
        let maan_client = MaanClient::new(PLATFORM_ID.to_string(), CERT_THUMBPRINT.to_string());
//         /*
// Номер счета: 40702810238030000904 
// БИК 046577964 
// Общество с ограниченной ответственностью "Петруня" 
// ИНН 6671217676 
// КПП 667101001
//          */
//         // let request = serde_json::json!({
//         //     "jsonrpc": "2.0",
//         //     "id": utils::new_uuid_v4().to_string(),
//         //     "method": "create_beneficiary_ul",
//         //     "params": {
//         //         "inn": "6671217676",
//         //         "nominal_account_code": "40702810620000088278",
//         //         "nominal_account_bic": "044525104",
//         //         "beneficiary_data": {
//         //             "name": "ООО \"Петруня\"",
//         //             "kpp": "667101001"
//         //         }
//         //     },
//         // });
//         // log::warn!("Create beneficiary request {:#?}", request);
//         // let request_bytes = serde_json::to_vec(&request).expect("failed serializing json value");        
//         // let response = maan_client.send_request(&signer, request_bytes).expect("failed");
//         // log::warn!("Response: {:#?}", response.json::<CreateBeneficiaryResponse>());

        // Beneficiary id - ca5cc32e-f4a4-460f-97a4-28e924a126ea
        // #[derive(Debug, Deserialize)]
        // struct ListBeneficiaries {
        //     jsonrpc: String,
        //     id: String,
        //     result: serde_json::Value,
        // }
        // let request = serde_json::json!({
        //     "jsonrpc": "2.0",
        //     "id": utils::new_uuid_v4().to_string(),
        //     "method": "list_beneficiary",
        //     "params": {
        //         "filters": {
        //             "legal_type": "J"
        //         },
        //     },
        // });
        // log::warn!("List beneficiaries request {:#?}", request);
        // let request_bytes = serde_json::to_vec(&request).expect("failed serializing json value");
        // let response = maan_client.send_request(&signer, request_bytes).expect("failed");
        // log::warn!("Response: {:#?}", response.json::<ListBeneficiaries>());

//         // mock payment
//         // #[derive(Debug, Deserialize)]
//         // struct TransferMoney {
//         //     jsonrpc: String,
//         //     id: String,
//         //     result: serde_json::Value,
//         // }
//         // let request = serde_json::json!({
//         //     "jsonrpc": "2.0",
//         //     "id": utils::new_uuid_v4().to_string(),
//         //     "method": "transfer_money",
//         //     "params": {
//         //         "recipient_account": "40702810620000088278",
//         //         "recipient_bank_code": "044525104",
//         //         "amount": 1000,
//         //         "purpose": "Оплата по договору 12345"
//         //     },
//         // });
//         // log::warn!("Transfer money request {:#?}", request);
//         // let request_bytes = serde_json::to_vec(&request).expect("failed serializing json value");
//         // let response = maan_client.send_request_tenders(&signer, request_bytes).expect("failed");
//         // log::warn!("Response: {:#?}", response.json::<TransferMoney>());

        // list payments
        #[derive(Debug, Deserialize)]
        struct ListPayments {
            jsonrpc: String,
            id: String,
            result: serde_json::Value,
        }
        // let request = serde_json::json!({
        //     "jsonrpc": "2.0",
        //     "id": utils::new_uuid_v4().to_string(),
        //     "method": "list_payments",
        //     "params": {
        //         "filters": {
        //             "incoming": true,
        //             "identify": false,
        //         },
        //     },
        // });
        // log::warn!("List payments request {:#?}", request);
        // let request_bytes = serde_json::to_vec(&request).expect("failed serializing json value");
        // let response = maan_client.send_request(&signer, request_bytes).expect("failed");
        // log::warn!("Response: {:#?}", response.json::<ListPayments>());

        // get payment
        #[derive(Debug, Deserialize)]
        struct GetPayment {
            jsonrpc: String,
            id: String,
            result: serde_json::Value,
        }
//         let request = serde_json::json!({
//             "jsonrpc": "2.0",
//             "id": utils::new_uuid_v4().to_string(),
//             "method": "get_payment",
//             "params": {
//                 "payment_id": "tender-helpers-c553dee87f834605ad9fe26bac58fdf4"
//             },
//         });
//         log::warn!("Get payment request {:#?}", request);
//         let request_bytes = serde_json::to_vec(&request).expect("failed serializing json value");
//         let response = maan_client.send_request(&signer, request_bytes).expect("failed");
//         log::warn!("Response: {:#?}", response.json::<GetPayment>());

//         /*
//         result: Object {
//                 "payment": Object {
//                     "amount": Number(1000.0),
//                     "created_at": String("2024-12-23T21:25:13.130522+03:00"),
//                     "deal_id": Null,
//                     "document_date": String("2024-12-23"),
//                     "document_number": String("83192"),
//                     "id": String("tender-helpers-c553dee87f834605ad9fe26bac58fdf4"),
//                     "identify": Bool(false),
//                     "incoming": Bool(true),
//                     "payer": Object {
//                         "account": String("40702810713500000456"),
//                         "bank_code": String("044525104"),
//                         "bank_correspondent_account": String("30101810745374525104"),
//                         "bank_name": String("ООО \"Банк Точка\""),
//                         "name": String("ООО \"ЕВРОСТИЛЬ\""),
//                         "tax_code": String("7203174679"),
//                         "tax_reason_code": String("0"),
//                     },
//                     "purpose": String("Оплата по договору 12345"),
//                     "recipient": Object {
//                         "account": String("40702810620000088278"),
//                         "bank_code": String("044525104"),
//                         "bank_correspondent_account": String("30101810745374525104"),
//                         "bank_name": String("ООО \"Банк Точка\""),
//                         "name": String("ООО \"ЭЛИТСТРОЙМОНТАЖ\""),
//                         "tax_code": String("2631037888"),
//                         "tax_reason_code": Null,
//                     },
//                     "status": String("PAID"),
//                     "type": String("incoming_unrecognized"),
//                     "updated_at": String("2024-12-23T21:57:48.783844+03:00"),
//                 },
//             },

        
//          */

        // // qr-code payment create
        // #[derive(Debug, Deserialize)]
        // struct GenerateSbpQrCode {
        //     jsonrpc: String,
        //     id: String,
        //     result: serde_json::Value,
        // }
        // let request = serde_json::json!({
        //     "jsonrpc": "2.0",
        //     "id": utils::new_uuid_v4().to_string(),
        //     "method": "generate_sbp_qrcode",
        //     "params": {
        //         "amount": 1000,
        //         "purpose": "Оплата по договору 67890",
        //         "nominal_account_code": "40702810620000088278",
        //         "nominal_account_bic": "044525104",
        //         "width": 400,
        //         "height": 400,
        //     },
        // });
        // log::warn!("Generate SBP QR code request {:#?}", request);
        // let request_bytes = serde_json::to_vec(&request).expect("failed serializing json value");
        // let response = maan_client.send_request(&signer, request_bytes).expect("failed");
        // log::warn!("Response: {:#?}", response.json::<GenerateSbpQrCode>());
        // qr code id - BD10003L8T8EKULS8F9AHGHGP5BDDE0G

//         // qr-code payment
        // #[derive(Debug, Deserialize)]
        // struct SendQrCodePayment {
        //     jsonrpc: String,
        //     id: String,
        //     result: serde_json::Value,
        // }
        // let request = serde_json::json!({
        //     "jsonrpc": "2.0",
        //     "id": utils::new_uuid_v4().to_string(),
        //     "method": "send_c2b_credit_transfer_request",
        //     "params": {
        //         "qrc_id": "BD10003L8T8EKULS8F9AHGHGP5BDDE0G",
        //         "amount": 1000,
        //         "qrc_type": "02",
        //         "creditor_bank_id": "100000000284",
        //     },
        // });
        // log::warn!("Send QR code payment request {:#?}", request);
        // let request_bytes = serde_json::to_vec(&request).expect("failed serializing json value");
        // let response = maan_client.send_request_tenders(&signer, request_bytes).expect("failed");
        // log::warn!("Response: {:#?}", response.json::<SendQrCodePayment>());

        // list payments again
        // let request = serde_json::json!({
        //     "jsonrpc": "2.0",
        //     "id": utils::new_uuid_v4().to_string(),
        //     "method": "list_payments",
        //     "params": {
        //         "filters": {
        //             "incoming": true,
        //             "identify": false,
        //         },
        //     },
        // });
        // log::warn!("List payments request {:#?}", request);
        // let request_bytes = serde_json::to_vec(&request).expect("failed serializing json value");
        // let response = maan_client.send_request(&signer, request_bytes).expect("failed");
        // log::warn!("Response: {:#?}", response.json::<ListPayments>());

//         // sbp payment id: "cbs-tb-92-466713629"
        // let request = serde_json::json!({
        //     "jsonrpc": "2.0",
        //     "id": utils::new_uuid_v4().to_string(),
        //     "method": "get_payment",
        //     "params": {
        //         "payment_id": "cbs-tb-92-466713629"
        //     },
        // });
        // log::warn!("Get payment request {:#?}", request);
        // let request_bytes = serde_json::to_vec(&request).expect("failed serializing json value");
        // let response = maan_client.send_request(&signer, request_bytes).expect("failed");
        // log::warn!("Response: {:#?}", response.json::<GetPayment>());
//         /*
//         result: Object {
//             "payment": Object {
//                 "amount": Number(1000.0),
//                 "created_at": String("2024-12-24T20:03:23.020022+03:00"),
//                 "deal_id": Null,
//                 "document_date": String("2024-12-24"),
//                 "document_number": String("50"),
//                 "id": String("cbs-tb-92-466713232"),
//                 "identify": Bool(false),
//                 "incoming": Bool(true),
//                 "payer": Object {
//                     "account": String("30233810620000000010"),
//                     "bank_code": String("044525104"),
//                     "bank_correspondent_account": String("30101810745374525104"),
//                     "bank_name": String("ООО \"Банк Точка\""),
//                     "name": String("ООО \"Банк Точка\""),
//                     "tax_code": String("9721194461"),
//                     "tax_reason_code": String("997950001"),
//                 },
//                 "purpose": String("Зачисление по QR коду ID AD100039RRK7QQB58GOQMKS10H76N2SD от 24.12.2024. НДС не предусмотрен."),
//                 "qrcode_id": String("AD100039RRK7QQB58GOQMKS10H76N2SD"),
//                 "recipient": Object {
//                     "account": String("40702810620000088278"),
//                     "bank_code": String("044525104"),
//                     "bank_correspondent_account": String("30101810745374525104"),
//                     "bank_name": String("ООО \"Банк Точка\""),
//                     "name": String("ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ \"ЭЛИТСТРОЙМОНТАЖ\""),
//                     "tax_code": String("2631037888"),
//                     "tax_reason_code": Null,
//                 },
//                 "status": String("PAID"),
//                 "type": String("incoming_sbp"),
//                 "updated_at": String("2024-12-24T20:03:23.020027+03:00"),
//             },
//         */
// //         let mut file_body = r#"<?xml version="1.0" encoding="UTF-8"?>
// // <note>
// //   <to>Tove 電紅 Какой-то кириллический текст</to>
// //   <from>Jani</from>
// //   <heading>Reminder</heading>
// //   <body>Don't forget me this weekend!</body>
// // </note>"#
// //         .trim()
// //         .as_bytes()
// //         .to_vec();

// //         let mut query_params = HashMap::new();
// //         query_params.insert("beneficiary_id", "ca5cc32e-f4a4-460f-97a4-28e924a126ea".to_string());
// //         query_params.insert("document_type", "contract_offer".to_string());
// //         query_params.insert("document_number", "12345".to_string());
// //         query_params.insert("document_date", "2024-12-24".to_string());

// //         let b64_body_signature = {
// //             let signed_body = signer.sign_raw_data(&file_body).unwrap();
// //             utils::base64_encode(signed_body)
// //         };

// //         let client = Client::new();
// //         let request = client
// //             .request(Method::POST, "https://pre.tochka.com/api/v1/cyclops/upload_document/beneficiary")
// //             .query(&query_params)
// //             .header("sign-system", PLATFORM_ID)
// //             .header("sign-thumbprint", CERT_THUMBPRINT)
// //             .header("sign-data", &b64_body_signature)
// //             .header("Content-Type", "text/xml")
// //             .body(file_body);

// //         log::trace!("Sending request: {:#?}", request);
// //         let resp = request.send().expect("failed");
// //         log::warn!("Response: {:#?}", resp.json::<serde_json::Value>());

//         // document id "cyclops-241224175417527-fbb640d5-1e32-441f-82d2-238310165246"
//         #[derive(Debug, Deserialize)]
//         struct GetDocument {
//             jsonrpc: String,
//             id: String,
//             result: serde_json::Value,
//         }
//         let request = serde_json::json!({
//             "jsonrpc": "2.0",
//             "id": utils::new_uuid_v4().to_string(),
//             "method": "get_document",
//             "params": {
//                 "document_id": "cyclops-241224175417527-fbb640d5-1e32-441f-82d2-238310165246"
//             },
//         });
//         let request_bytes = serde_json::to_vec(&request).expect("failed serializing json value");
//         let response = maan_client.send_request(&signer, request_bytes).expect("failed");
//         log::warn!("Response: {:#?}", response.json::<GetDocument>());

    //     #[derive(Debug, Deserialize)]
    //     struct CreateVirtualAccount {
    //         jsonrpc: String,
    //         id: String,
    //         result: serde_json::Value,
    //     }
    //     let request = serde_json::json!({
    //         "jsonrpc": "2.0",
    //         "id": utils::new_uuid_v4().to_string(),
    //         "method": "create_virtual_account",
    //         "params": {
    //             "beneficiary_id": "ca5cc32e-f4a4-460f-97a4-28e924a126ea",
    //         },
    //     });
    //     let request_bytes = serde_json::to_vec(&request).expect("failed serializing json value");
    //     let response = maan_client.send_request(&signer, request_bytes).expect("failed");
    //     log::warn!("Response: {:#?}", response.json::<CreateVirtualAccount>());

        // virtual account - e8449e0f-b18a-423a-a3fd-dd1c2807e90d
        // #[derive(Debug, Deserialize)]
        // struct IdentificationPayment {
        //     jsonrpc: String,
        //     id: String,
        //     result: serde_json::Value,
        // }
        // let request = serde_json::json!({
        //     "jsonrpc": "2.0",
        //     "id": utils::new_uuid_v4().to_string(),
        //     "method": "identification_payment",
        //     "params": {
        //         "payment_id": "cbs-tb-92-466713629",
        //         "owners": [{
        //             "virtual_account": "e8449e0f-b18a-423a-a3fd-dd1c2807e90d",
        //             "amount": 1000
        //         }],
        //     },
        // });
        // let request_bytes = serde_json::to_vec(&request).expect("failed serializing json value");
        // let response = maan_client.send_request(&signer, request_bytes).expect("failed");
        // log::warn!("Response: {:#?}", response.json::<serde_json::Value>());

        #[derive(Debug, Deserialize)]
        struct GetVirtualAccount {
            jsonrpc: String,
            id: String,
            result: serde_json::Value,
        }
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": utils::new_uuid_v4().to_string(),
            "method": "get_virtual_account",
            "params": {
                "virtual_account": "e8449e0f-b18a-423a-a3fd-dd1c2807e90d",
            },
        });
        let request_bytes = serde_json::to_vec(&request).expect("failed serializing json value");
        let response = maan_client.send_request(&signer, request_bytes).expect("failed");
        log::warn!("Response: {:#?}", response.json::<serde_json::Value>());
    }
}
