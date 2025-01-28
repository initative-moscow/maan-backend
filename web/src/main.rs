// TODO:
// 1. use anyhow for the error

mod db;
mod error;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use db::{InMemoryStore, Store};
use error::AnyhowResponseError;
use maan_core::tochka::{
    create_beneficiary::{CreateBeneficiaryResponse, CreateBeneficiaryUlRequest},
    create_deal::{CreateDealRequest, CreateDealResponse},
    create_virtual_account::{CreateVirtualAccountRequest, CreateVirtualAccountResponse},
    execute_deal::{ExecuteDealRequest, ExecuteDealResponse},
    get_beneficiary::{GetBeneficiaryRequest, GetBeneficiaryResponse},
    get_deal::{GetDealRequest, GetDealResponse},
    get_document::{GetDocumentRequest, GetDocumentResponse, GetDocumentResponseIO},
    get_payment::{GetPaymentRequest, GetPaymentResponseIO},
    get_virtual_account::{GetVirtualAccountRequest, GetVirtualAccountResponseIO},
    identification_payment::{IdentificationPaymentRequest, IdentificationPaymentResponse},
    list_beneficiary::{ListBeneficiaryRequest, ListBeneficiaryResponse},
    list_deals::{ListDealsReponse, ListDealsRequest},
    list_payments::{ListPaymentsRequest, ListPaymentsResponse},
    list_virtual_account::{ListVirtualAccountRequest, ListVirtualAccountResponse},
    sbp_qrcode::{GenerateSbpQrCodeRequest, GenerateSbpQrCodeResponseIO},
    update_deal::{UpdateDealRequest, UpdateDealResponse},
    TochkaApiResponse, TochkaApiResponsePayload,
};
use maan_core::{MaanClient, Signer};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, fs, path::PathBuf};
use tokio::sync::Mutex;

struct AppData {
    store: Mutex<Box<dyn Store>>,
    maan_client: MaanClient,
    signer: Signer,
}

#[post("/create_beneficiary_ul")]
async fn create_beneficiary_ul(
    data: web::Data<AppData>,
    create_beneficiary_req: web::Json<CreateBeneficiaryUlRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let beneficiary_data = create_beneficiary_req.0.beneficiary_data.clone();
    let data_clone = data.clone();
    let task = async move {
        let params = serde_json::to_value(&create_beneficiary_req.0)?;
        let req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": maan_core::utils::new_uuid_v4().to_string(),
            "method": "create_beneficiary_ul",
            "params": params,
        });
        log::debug!("Sending request {req:#?}");
        let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
        web::block(move || {
            data_clone
                .maan_client
                .send_request(&data_clone.signer, bytes)
                .unwrap()
                .json::<TochkaApiResponse<CreateBeneficiaryResponse>>()
        })
        .await
        .expect("web::block failed")
        .map_err(anyhow::Error::from)
    };

    let res = task.await?;
    match res.payload {
        TochkaApiResponsePayload::Result { result } => {
            let CreateBeneficiaryResponse::Beneficiary { id, .. } = &result;
            data.store
                .lock()
                .await
                .store_beneficiary(id.clone(), beneficiary_data)
                .await?;

            Ok(HttpResponse::Ok().json(result))
        }
        TochkaApiResponsePayload::Error { error } => {
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

#[get("/list_beneficiary")]
async fn list_beneficiary(
    data: web::Data<AppData>,
    list_beneficiary_req: web::Json<ListBeneficiaryRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let params = serde_json::to_value(&list_beneficiary_req.0).map_err(anyhow::Error::from)?;
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": maan_core::utils::new_uuid_v4().to_string(),
        "method": "list_beneficiary",
        "params": params,
    });
    log::debug!("Sending request {req:#?}");
    let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    let res = web::block(move || {
        data.maan_client
            .send_request(&data.signer, bytes)
            .unwrap()
            .json::<TochkaApiResponse<ListBeneficiaryResponse>>()
    })
    .await
    .expect("web::block failed")
    .map_err(anyhow::Error::from)?;

    match res.payload {
        TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
        TochkaApiResponsePayload::Error { error } => {
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

#[get("/get_beneficiary")]
async fn get_beneficiary(
    data: web::Data<AppData>,
    get_beneficiary_req: web::Json<GetBeneficiaryRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let params = serde_json::to_value(&get_beneficiary_req.0).map_err(anyhow::Error::from)?;
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": maan_core::utils::new_uuid_v4().to_string(),
        "method": "get_beneficiary",
        "params": params,
    });
    log::debug!("Sending request {req:#?}");
    let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    let res = web::block(move || {
        let res = data.maan_client.send_request(&data.signer, bytes).unwrap();
        let resp_json = res
            .json::<serde_json::Value>()
            .map_err(anyhow::Error::from)?;
        log::debug!("Get beneficiary resp {resp_json:#?}");

        serde_json::from_value::<TochkaApiResponse<GetBeneficiaryResponse>>(resp_json)
            .map_err(anyhow::Error::from)
    })
    .await
    .expect("web::block failed")?;

    match res.payload {
        TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
        TochkaApiResponsePayload::Error { error } => {
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

#[post("/generate_sbp_qrcode")]
async fn generate_sbp_qrcode(
    data: web::Data<AppData>,
    generate_qrcode_req: web::Json<GenerateSbpQrCodeRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let params = serde_json::to_value(&generate_qrcode_req.0).map_err(anyhow::Error::from)?;
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": maan_core::utils::new_uuid_v4().to_string(),
        "method": "generate_sbp_qrcode",
        "params": params,
    });
    log::debug!("Sending request {req:#?}");
    let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    let res = web::block(move || {
        data.maan_client
            .send_request(&data.signer, bytes)
            .unwrap()
            .json::<TochkaApiResponse<GenerateSbpQrCodeResponseIO>>()
    })
    .await
    .expect("web::block failed")
    .map_err(anyhow::Error::from)?;

    match res.payload {
        TochkaApiResponsePayload::Error { error } => {
            Ok(HttpResponse::InternalServerError().json(error))
        }
        TochkaApiResponsePayload::Result { result } => {
            Ok(HttpResponse::Ok().json(result.into_inner()))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TestSendPaymentResponse {
    service_pay_key: String,
    status: String,
}

#[post("/test_send_payment/{amount}")]
async fn test_send_payment(
    data: web::Data<AppData>,
    path: web::Path<(u32,)>,
) -> Result<impl Responder, AnyhowResponseError> {
    let (amount,) = path.into_inner();
    let params = serde_json::json!({
        "amount": amount,
        "purpose": "testing",
        "recipient_account": "40702810620000088278",
        "recipient_bank_code": "044525104",

    });
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": maan_core::utils::new_uuid_v4().to_string(),
        "method": "transfer_money",
        "params": params,
    });
    log::debug!("Sending request {req:#?}");
    let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    let res = web::block(move || {
        data.maan_client
            .send_request_tenders(&data.signer, bytes)
            .unwrap()
            .json::<TochkaApiResponse<TestSendPaymentResponse>>()
    })
    .await
    .expect("web::block failed")
    .map_err(anyhow::Error::from)?;

    match res.payload {
        TochkaApiResponsePayload::Error { error } => {
            Ok(HttpResponse::InternalServerError().json(error))
        }
        TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TestSendQrPaymentRequest {
    pub amount: u32,
    pub qrc_type: String,
    pub qrc_id: String,
    pub creditor_bank_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestSendQrPaymentResponse {
    pub transaction_id: String,
}

#[post("/test_send_qr_payment/{amount}/{qrc_id}")]
async fn test_send_qr_payment(
    data: web::Data<AppData>,
    path: web::Path<(u32, String)>,
) -> Result<impl Responder, AnyhowResponseError> {
    let (amount, qrc_id) = path.into_inner();
    let req = TestSendQrPaymentRequest {
        amount,
        qrc_type: "02".to_string(),
        qrc_id,
        creditor_bank_id: "100000000284".to_string(),
    };
    let params = serde_json::to_value(&req).map_err(anyhow::Error::from)?;
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": maan_core::utils::new_uuid_v4().to_string(),
        "method": "send_c2b_credit_transfer_request",
        "params": params,
    });
    log::debug!("Sending request {req:#?}");
    let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    let res = web::block(move || {
        data.maan_client
            .send_request_tenders(&data.signer, bytes)
            .unwrap()
            .json::<TochkaApiResponse<TestSendQrPaymentResponse>>()
    })
    .await
    .expect("web::block failed")
    .map_err(anyhow::Error::from)?;

    match res.payload {
        TochkaApiResponsePayload::Error { error } => {
            Ok(HttpResponse::InternalServerError().json(error))
        }
        TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
    }
}

// TODO possibly must be done in a separate thread after creating qrcode payment

#[get("/list_payments")]
async fn list_payments(
    data: web::Data<AppData>,
    list_payments_req: web::Json<ListPaymentsRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let params = serde_json::to_value(&list_payments_req.0).map_err(anyhow::Error::from)?;
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": maan_core::utils::new_uuid_v4().to_string(),
        "method": "list_payments",
        "params": params,
    });
    log::debug!("Sending request {req:#?}");
    let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    let res = web::block(move || {
        let res = data.maan_client.send_request(&data.signer, bytes).unwrap();
        let resp_json = res
            .json::<serde_json::Value>()
            .map_err(anyhow::Error::from)?;
        log::debug!("List payment received this response {resp_json:#?}");

        serde_json::from_value::<TochkaApiResponse<ListPaymentsResponse>>(resp_json)
            .map_err(anyhow::Error::from)
    })
    .await
    .expect("web::block failed")?;

    match res.payload {
        TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
        TochkaApiResponsePayload::Error { error } => {
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

#[get("/get_payment")]
async fn get_payment(
    data: web::Data<AppData>,
    get_payment_req: web::Json<GetPaymentRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let params = serde_json::to_value(&get_payment_req.0).map_err(anyhow::Error::from)?;
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": maan_core::utils::new_uuid_v4().to_string(),
        "method": "get_payment",
        "params": params,
    });
    log::debug!("Sending request {req:#?}");
    let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    let res = web::block(move || {
        let res = data.maan_client.send_request(&data.signer, bytes).unwrap();
        let resp_json = res
            .json::<serde_json::Value>()
            .map_err(anyhow::Error::from)?;
        log::debug!("Get payment received this response {resp_json:#?}");

        serde_json::from_value::<TochkaApiResponse<GetPaymentResponseIO>>(resp_json)
            .map_err(anyhow::Error::from)
    })
    .await
    .expect("web::block failed")?;

    match res.payload {
        TochkaApiResponsePayload::Result { result } => {
            Ok(HttpResponse::Ok().json(result.into_inner()))
        }
        TochkaApiResponsePayload::Error { error } => {
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

/*
PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPG5vdGU+Cjx0bz5Ub3ZlIOmbu+e0hSDQmtCw0LrQvtC5LdGC0L4g0LrQuNGA0LjQu9C70LjRh9C10YHQutC40Lkg0YLQtdC60YHRgjwvdG8+Cjxmcm9tPkphbmk8L2Zyb20+CjxoZWFkaW5nPlJlbWluZGVyPC9oZWFkaW5nPgo8Ym9keT5Eb24ndCBmb3JnZXQgbWUgdGhpcyB3ZWVrZW5kITwvYm9keT4KPC9ub3RlPiI=
*/

#[derive(Debug, Serialize, Deserialize)]
struct UploadDocumentBeneficiaryRequest {
    b64_document: String,
    beneficiary_id: String,
    document_number: String,
    document_date: String,
    content_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UploadDocumentBeneficiaryResponse {
    document_id: String,
}

#[post("/upload_document_beneficiary")]
async fn upload_document_beneficiary(
    data: web::Data<AppData>,
    upload_document_req: web::Json<UploadDocumentBeneficiaryRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let UploadDocumentBeneficiaryRequest {
        b64_document,
        beneficiary_id,
        document_number,
        document_date,
        content_type,
    } = upload_document_req.0;
    let res = web::block(move || {
        let resp = data
            .maan_client
            .upload_document_beneficiary(
                &data.signer,
                b64_document,
                beneficiary_id,
                document_number,
                document_date,
                content_type,
            )
            .unwrap();
        let resp_json = resp.json::<serde_json::Value>()?;
        log::debug!("Upload document beneficiary resp - {resp_json:#?}");

        serde_json::from_value::<UploadDocumentDealResponse>(resp_json).map_err(anyhow::Error::from)
    })
    .await
    .unwrap()
    .map_err(anyhow::Error::from)?;

    Ok(HttpResponse::Ok().json(res))
}

#[get("/get_document")]
async fn get_document(
    data: web::Data<AppData>,
    get_document_req: web::Json<GetDocumentRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let params = serde_json::to_value(&get_document_req.0).map_err(anyhow::Error::from)?;
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": maan_core::utils::new_uuid_v4().to_string(),
        "method": "get_document",
        "params": params,
    });
    log::debug!("Sending request {req:#?}");
    let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    let res = web::block(move || {
        data.maan_client
            .send_request(&data.signer, bytes)
            .unwrap()
            .json::<TochkaApiResponse<GetDocumentResponseIO>>()
    })
    .await
    .expect("web::block failed")
    .map_err(anyhow::Error::from)?;

    match res.payload {
        TochkaApiResponsePayload::Result { result } => {
            Ok(HttpResponse::Ok().json(result.into_inner()))
        }
        TochkaApiResponsePayload::Error { error } => {
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

#[post("/create_virtual_account")]
async fn create_virtual_account(
    data: web::Data<AppData>,
    create_acc_req: web::Json<CreateVirtualAccountRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let params = serde_json::to_value(&create_acc_req.0).map_err(anyhow::Error::from)?;
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": maan_core::utils::new_uuid_v4().to_string(),
        "method": "create_virtual_account",
        "params": params,
    });
    log::debug!("Sending request {req:#?}");
    let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    let res = web::block(move || {
        data.maan_client
            .send_request(&data.signer, bytes)
            .unwrap()
            .json::<TochkaApiResponse<CreateVirtualAccountResponse>>()
    })
    .await
    .expect("web::block failed")
    .map_err(anyhow::Error::from)?;

    match res.payload {
        TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
        TochkaApiResponsePayload::Error { error } => {
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

#[get("/get_virtual_account")]
async fn get_virtual_account(
    data: web::Data<AppData>,
    get_virtual_acc_req: web::Json<GetVirtualAccountRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let params = serde_json::to_value(&get_virtual_acc_req.0).map_err(anyhow::Error::from)?;
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": maan_core::utils::new_uuid_v4().to_string(),
        "method": "get_virtual_account",
        "params": params,
    });
    log::debug!("Sending request {req:#?}");
    let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    let res = web::block(move || {
        let res = data.maan_client.send_request(&data.signer, bytes).unwrap();
        let resp_json = res
            .json::<serde_json::Value>()
            .map_err(anyhow::Error::from)?;
        log::debug!("Get virtual account - {resp_json:#?}");

        serde_json::from_value::<TochkaApiResponse<GetVirtualAccountResponseIO>>(resp_json)
            .map_err(anyhow::Error::from)
    })
    .await
    .expect("web::block failed")?;

    match res.payload {
        TochkaApiResponsePayload::Result { result } => {
            Ok(HttpResponse::Ok().json(result.into_inner()))
        }
        TochkaApiResponsePayload::Error { error } => {
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

#[get("/list_virtual_account")]
async fn list_virtual_account(
    data: web::Data<AppData>,
    list_virtual_account_req: web::Json<ListVirtualAccountRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let params = serde_json::to_value(&list_virtual_account_req.0).map_err(anyhow::Error::from)?;
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": maan_core::utils::new_uuid_v4().to_string(),
        "method": "list_virtual_account",
        "params": params,
    });
    log::debug!("Sending request {req:#?}");
    let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    let res = web::block(move || {
        data.maan_client
            .send_request(&data.signer, bytes)
            .unwrap()
            .json::<TochkaApiResponse<ListVirtualAccountResponse>>()
    })
    .await
    .expect("web::block failed")
    .map_err(anyhow::Error::from)?;

    match res.payload {
        TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
        TochkaApiResponsePayload::Error { error } => {
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

#[post("/identify_payment")]
async fn identify_payment(
    data: web::Data<AppData>,
    identify_payment_req: web::Json<IdentificationPaymentRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let params = serde_json::to_value(&identify_payment_req.0).map_err(anyhow::Error::from)?;
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": maan_core::utils::new_uuid_v4().to_string(),
        "method": "identification_payment",
        "params": params,
    });
    log::debug!("Sending request {req:#?}");
    let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    let res = web::block(move || {
        let resp = data.maan_client.send_request(&data.signer, bytes).unwrap();

        let resp_json = resp
            .json::<serde_json::Value>()
            .map_err(anyhow::Error::from)?;
        log::debug!("Identify payment resp - {resp_json:#?}");

        serde_json::from_value::<TochkaApiResponse<IdentificationPaymentResponse>>(resp_json)
            .map_err(anyhow::Error::from)
    })
    .await
    .expect("web::block failed")?;

    match res.payload {
        TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
        TochkaApiResponsePayload::Error { error } => {
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

#[post("/create_deal")]
async fn create_deal(
    data: web::Data<AppData>,
    create_deal_req: web::Json<CreateDealRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let params = serde_json::to_value(&create_deal_req.0).map_err(anyhow::Error::from)?;
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": maan_core::utils::new_uuid_v4().to_string(),
        "method": "create_deal",
        "params": params,
    });
    log::debug!("Sending request {req:#?}");
    let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    let res = web::block(move || {
        data.maan_client
            .send_request(&data.signer, bytes)
            .unwrap()
            .json::<TochkaApiResponse<CreateDealResponse>>()
    })
    .await
    .expect("web::block failed")
    .map_err(anyhow::Error::from)?;

    match res.payload {
        TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
        TochkaApiResponsePayload::Error { error } => {
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

#[post("/update_deal")]
async fn update_deal(
    data: web::Data<AppData>,
    update_deal_req: web::Json<UpdateDealRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let params = serde_json::to_value(&update_deal_req.0).map_err(anyhow::Error::from)?;
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": maan_core::utils::new_uuid_v4().to_string(),
        "method": "update_deal",
        "params": params,
    });
    log::debug!("Sending request {req:#?}");
    let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    let res = web::block(move || {
        let resp = data.maan_client.send_request(&data.signer, bytes).unwrap();

        let resp_json = resp
            .json::<serde_json::Value>()
            .map_err(anyhow::Error::from)?;
        log::debug!("Update deal resp - {resp_json:#?}");

        serde_json::from_value::<TochkaApiResponse<UpdateDealResponse>>(resp_json)
            .map_err(anyhow::Error::from)
    })
    .await
    .expect("web::block failed")
    .map_err(anyhow::Error::from)?;

    match res.payload {
        TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
        TochkaApiResponsePayload::Error { error } => {
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct UploadDocumentDealRequest {
    b64_document: String,
    beneficiary_id: String,
    deal_id: String,
    document_number: String,
    document_date: String,
    content_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UploadDocumentDealResponse {
    document_id: String,
}

#[post("/upload_document_deal")]
async fn upload_document_deal(
    data: web::Data<AppData>,
    upload_document_deal_req: web::Json<UploadDocumentDealRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let UploadDocumentDealRequest {
        b64_document,
        beneficiary_id,
        deal_id,
        document_number,
        document_date,
        content_type,
    } = upload_document_deal_req.0;
    let res = web::block(move || {
        let resp = data
            .maan_client
            .upload_document_deal(
                &data.signer,
                b64_document,
                beneficiary_id,
                deal_id,
                document_number,
                document_date,
                content_type,
            )
            .unwrap();
        let resp_json = resp.json::<serde_json::Value>()?;
        log::debug!("Upload document deal resp - {resp_json:#?}");

        serde_json::from_value::<UploadDocumentDealResponse>(resp_json).map_err(anyhow::Error::from)
    })
    .await
    .unwrap()
    .map_err(anyhow::Error::from)?;

    Ok(HttpResponse::Ok().json(res))
}

// TODO FIX IT!
#[get("/get_deal")]
async fn get_deal(
    data: web::Data<AppData>,
    get_deal_req: web::Json<GetDealRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let params = serde_json::to_value(&get_deal_req.0).map_err(anyhow::Error::from)?;
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": maan_core::utils::new_uuid_v4().to_string(),
        "method": "get_deal",
        "params": params,
    });
    log::debug!("Sending request {req:#?}");
    let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    let res = web::block(move || {
        let resp = data.maan_client.send_request(&data.signer, bytes).unwrap();

        let resp_json = resp
            .json::<serde_json::Value>()
            .map_err(anyhow::Error::from)?;
        log::debug!("Deal info {resp_json:#?}");

        serde_json::from_value::<TochkaApiResponse<GetDealResponse>>(resp_json)
            .map_err(anyhow::Error::from)
    })
    .await
    .expect("web::block failed")
    .map_err(anyhow::Error::from)?;

    match res.payload {
        TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
        TochkaApiResponsePayload::Error { error } => {
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

#[get("/list_deals")]
async fn list_deals(
    data: web::Data<AppData>,
    list_deals_req: web::Json<ListDealsRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let params = serde_json::to_value(&list_deals_req.0).map_err(anyhow::Error::from)?;
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": maan_core::utils::new_uuid_v4().to_string(),
        "method": "list_deals",
        "params": params,
    });
    log::debug!("Sending request {req:#?}");
    let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    let res = web::block(move || {
        let resp = data.maan_client.send_request(&data.signer, bytes).unwrap();

        let resp_json = resp
            .json::<serde_json::Value>()
            .map_err(anyhow::Error::from)?;
        log::debug!("List deals info {resp_json:#?}");

        serde_json::from_value::<TochkaApiResponse<ListDealsReponse>>(resp_json)
            .map_err(anyhow::Error::from)
    })
    .await
    .expect("web::block failed")
    .map_err(anyhow::Error::from)?;

    match res.payload {
        TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
        TochkaApiResponsePayload::Error { error } => {
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

// TODO:
// list deals
// разберись, почему сделка отменилась!

// TEST it
// Refactor
// Implement rest
// DB

#[post("/execute_deal")]
async fn execute_deal(
    data: web::Data<AppData>,
    deal_id_req: web::Json<ExecuteDealRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let params = serde_json::to_value(&deal_id_req.0).map_err(anyhow::Error::from)?;
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": maan_core::utils::new_uuid_v4().to_string(),
        "method": "execute_deal",
        "params": params,
    });
    log::debug!("Sending request {req:#?}");
    let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    let res = web::block(move || {
        data.maan_client
            .send_request(&data.signer, bytes)
            .unwrap()
            .json::<TochkaApiResponse<ExecuteDealResponse>>()
    })
    .await
    .expect("web::block failed")
    .map_err(anyhow::Error::from)?;

    match res.payload {
        TochkaApiResponsePayload::Error { error } => {
            Ok(HttpResponse::InternalServerError().json(error))
        }
        TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
    }
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let _ = env_logger::Builder::from_default_env()
        .format_module_path(false)
        .format_level(true)
        .try_init();

    let private_key_string = fs::read_to_string(args.private_key_path)?;

    let signer = Signer::new(private_key_string)?;
    let maan_client = MaanClient::new(args.sign_system, args.sign_thumbprint, args.endpoint);
    let store = {
        let store: Box<dyn Store> = Box::new(InMemoryStore::new());
        Mutex::new(store)
    };

    let data = web::Data::new(AppData {
        store,
        maan_client,
        signer,
    });

    // TODO: for wrong entries - return error
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(create_beneficiary_ul)
            .service(list_beneficiary)
            .service(generate_sbp_qrcode)
            .service(list_payments)
            .service(upload_document_beneficiary)
            .service(create_virtual_account)
            .service(identify_payment)
            .service(upload_document_deal)
            .service(execute_deal)
            .service(test_send_qr_payment)
            .service(test_send_payment)
            .service(get_payment)
            .service(list_virtual_account)
            .service(get_virtual_account)
            .service(get_document)
            .service(create_deal)
            .service(get_deal)
            .service(list_deals)
            .service(get_beneficiary)
            .service(update_deal)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .map_err(Into::into)
}

#[derive(Debug, Parser, Deserialize)]
#[clap(version = "1.0", about = "maan-backend web-server")]
struct Args {
    /// Path to the private key for signing requests to the Tochka bank.
    #[arg(long = "private-key")]
    private_key_path: PathBuf,

    /// Sign system for the Tochka.
    #[arg(long = "sign-system")]
    sign_system: String,

    /// Sign thumbprint for the Tochka.
    #[arg(long = "sign-thumbprint")]
    sign_thumbprint: String,

    /// Tochka service API endpoint.
    #[arg(long = "endpoint")]
    endpoint: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use anyhow::Context;
    use maan_core::tochka::{create_beneficiary::BeneficiaryData, TochkaError};

    const CONFIG_PATH: &str = "./.test_config.toml";

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    enum MaanWebResponse<T> {
        Error(TochkaError),
        Ok(T),
    }

    impl<T> MaanWebResponse<T> {
        fn is_ok(&self) -> bool {
            matches!(self, MaanWebResponse::Ok(_))
        }

        fn is_err(&self) -> bool {
            matches!(self, MaanWebResponse::Error(_))
        }
    }

    fn generate_random_inn_j() -> String {
        use rand::Rng;

        let base: String = (0..9)
            .map(|_| rand::thread_rng().gen_range(0..10).to_string())
            .collect();
        let coefficients = [2u32, 4, 10, 3, 5, 9, 4, 6, 8];
        let checksum = (base.as_str())
            .chars()
            .zip(coefficients.iter())
            .map(|(inn_char, coeff)| inn_char.to_digit(10).expect("invalid digit") * coeff)
            .sum::<u32>()
            % 11
            % 10;

        format!("{}{}", base, checksum)
    }

    #[tokio::test]
    async fn test_create_beneficiary_ul() -> anyhow::Result<()> {
        let _ = env_logger::Builder::from_default_env()
            .format_module_path(false)
            .format_level(true)
            .try_init();

        let args = {
            let config_content =
                fs::read_to_string(PathBuf::from(CONFIG_PATH)).context("can't read config file")?;
            toml::from_str::<Args>(&config_content).context("failed to parse into `Params`")?
        };

        let signer = {
            let private_key_string = fs::read_to_string(args.private_key_path)
                .context("failed to read private key file")?;
            Signer::new(private_key_string).context("failed to create signer")?
        };
        let maan_client = MaanClient::new(args.sign_system, args.sign_thumbprint, args.endpoint);
        let store = {
            let store: Box<dyn Store> = Box::new(InMemoryStore::new());
            Mutex::new(store)
        };

        let data = web::Data::new(AppData {
            store,
            maan_client,
            signer,
        });

        let app = test::init_service(
            App::new()
                .app_data(data.clone())
                .service(create_beneficiary_ul),
        )
        .await;

        let beneficiary_data = BeneficiaryData {
            name: "ООО \"Петруня\"".to_string(),
            kpp: "667101001".to_string(),
            ogrn: None,
            is_branch: None,
        };
        let req_body = CreateBeneficiaryUlRequest {
            inn: generate_random_inn_j(),
            nominal_account_code: "40702810620000088278".to_string(),
            nominal_account_bic: "044525104".to_string(),
            beneficiary_data: beneficiary_data.clone(),
        };
        let req = test::TestRequest::post()
            .uri("/create_beneficiary_ul")
            .set_json(&req_body)
            .to_request();

        let resp = test::call_service(&app, req).await;
        let resp_body: MaanWebResponse<CreateBeneficiaryResponse> =
            test::read_body_json(resp).await;
        let id = match resp_body {
            MaanWebResponse::Error(tochka_error) => {
                panic!("Failed to create beneficiary. Request - {req_body:#?}, got error: {tochka_error:#?}");
            }
            MaanWebResponse::Ok(v) => {
                let CreateBeneficiaryResponse::Beneficiary { id, .. } = v;

                id
            }
        };

        let maybe_beneficiary = data
            .store
            .lock()
            .await
            .get_beneficiary(&id)
            .await
            .expect("failed sending request");
        assert_eq!(maybe_beneficiary, Some(beneficiary_data));

        // Sending same data must fail, as beneficiary already exists
        let req = test::TestRequest::post()
            .uri("/create_beneficiary_ul")
            .set_json(&req_body)
            .to_request();
        let resp = test::call_service(&app, req).await;
        let resp_body: MaanWebResponse<CreateBeneficiaryResponse> =
            test::read_body_json(resp).await;
        assert!(resp_body.is_err());

        Ok(())
    }
}
