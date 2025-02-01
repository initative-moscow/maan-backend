// TODO:
// 1. use anyhow for the error

mod db;
mod error;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Context;
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

// TODO with a document upload later
// TODO input is just inn and beneficiary data
#[post("/create_beneficiary_ul")]
async fn create_beneficiary_ul(
    data: web::Data<AppData>,
    create_beneficiary_req: web::Json<CreateBeneficiaryUlRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let beneficiary_data = create_beneficiary_req.0.beneficiary_data.clone();
    let params = serde_json::to_value(&create_beneficiary_req.0).map_err(anyhow::Error::from)?;
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": maan_core::utils::new_uuid_v4().to_string(),
        "method": "create_beneficiary_ul",
        "params": params,
    });

    log::debug!("Sending request {req:#?}");

    let req_bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    let resp = data
        .maan_client
        .send_request(&data.signer, req_bytes)
        .await
        .context("failed sending the create_beneficiary_ul request")
        .map_err(anyhow::Error::from)?;
    let json_resp = resp
        .json::<TochkaApiResponse<CreateBeneficiaryResponse>>()
        .await
        .context("failed decoding the create_beneficiary_ul response")
        .map_err(anyhow::Error::from)?;

    match json_resp.payload {
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

// TODO [IMPORTANT] implement get_/list_ or_update logic for list_\get_beneficiary
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

    let req_bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    let resp = data
        .maan_client
        .send_request(&data.signer, req_bytes)
        .await
        .context("failed sending the list_beneficiary request")
        .map_err(anyhow::Error::from)?;
    let json_resp = resp
        .json::<TochkaApiResponse<ListBeneficiaryResponse>>()
        .await
        .context("failed decoding the list_beneficiary response")
        .map_err(anyhow::Error::from)?;

    match json_resp.payload {
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

    let req_bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    let resp = data
        .maan_client
        .send_request(&data.signer, req_bytes)
        .await
        .context("failed sending the get_beneficiary request")?;
    let resp_json = resp
        .json::<serde_json::Value>()
        .await
        .context("failed decoding the get_beneficiary response to `serde_json::Value`")
        .map_err(anyhow::Error::from)?;

    log::debug!("Get beneficiary raw - response {resp_json:#?}");

    let resp_json = serde_json::from_value::<TochkaApiResponse<GetBeneficiaryResponse>>(resp_json)
        .context("failed decoding the get_beneficiary response into `GetBeneficiaryResponse`")
        .map_err(anyhow::Error::from)?;

    match resp_json.payload {
        TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
        TochkaApiResponsePayload::Error { error } => {
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

// #[post("/generate_sbp_qrcode")]
// async fn generate_sbp_qrcode(
//     data: web::Data<AppData>,
//     generate_qrcode_req: web::Json<GenerateSbpQrCodeRequest>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     let params = serde_json::to_value(&generate_qrcode_req.0).map_err(anyhow::Error::from)?;
//     let req = serde_json::json!({
//         "jsonrpc": "2.0",
//         "id": maan_core::utils::new_uuid_v4().to_string(),
//         "method": "generate_sbp_qrcode",
//         "params": params,
//     });
//     log::debug!("Sending request {req:#?}");
//     let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let res = web::block(move || {
//         data.maan_client
//             .send_request(&data.signer, bytes)
//             .unwrap()
//             .json::<TochkaApiResponse<GenerateSbpQrCodeResponseIO>>()
//     })
//     .await
//     .expect("web::block failed")
//     .map_err(anyhow::Error::from)?;

//     match res.payload {
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//         TochkaApiResponsePayload::Result { result } => {
//             Ok(HttpResponse::Ok().json(result.into_inner()))
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct TestSendPaymentResponse {
//     service_pay_key: String,
//     status: String,
// }

// #[post("/test_send_payment/{amount}")]
// async fn test_send_payment(
//     data: web::Data<AppData>,
//     path: web::Path<(u32,)>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     let (amount,) = path.into_inner();
//     let params = serde_json::json!({
//         "amount": amount,
//         "purpose": "testing",
//         "recipient_account": "40702810620000088278",
//         "recipient_bank_code": "044525104",

//     });
//     let req = serde_json::json!({
//         "jsonrpc": "2.0",
//         "id": maan_core::utils::new_uuid_v4().to_string(),
//         "method": "transfer_money",
//         "params": params,
//     });
//     log::debug!("Sending request {req:#?}");
//     let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let res = web::block(move || {
//         data.maan_client
//             .send_request_tenders(&data.signer, bytes)
//             .unwrap()
//             .json::<TochkaApiResponse<TestSendPaymentResponse>>()
//     })
//     .await
//     .expect("web::block failed")
//     .map_err(anyhow::Error::from)?;

//     match res.payload {
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//         TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct TestSendQrPaymentRequest {
//     pub amount: u32,
//     pub qrc_type: String,
//     pub qrc_id: String,
//     pub creditor_bank_id: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct TestSendQrPaymentResponse {
//     pub transaction_id: String,
// }

// #[post("/test_send_qr_payment/{amount}/{qrc_id}")]
// async fn test_send_qr_payment(
//     data: web::Data<AppData>,
//     path: web::Path<(u32, String)>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     let (amount, qrc_id) = path.into_inner();
//     let req = TestSendQrPaymentRequest {
//         amount,
//         qrc_type: "02".to_string(),
//         qrc_id,
//         creditor_bank_id: "100000000284".to_string(),
//     };
//     let params = serde_json::to_value(&req).map_err(anyhow::Error::from)?;
//     let req = serde_json::json!({
//         "jsonrpc": "2.0",
//         "id": maan_core::utils::new_uuid_v4().to_string(),
//         "method": "send_c2b_credit_transfer_request",
//         "params": params,
//     });
//     log::debug!("Sending request {req:#?}");
//     let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let res = web::block(move || {
//         data.maan_client
//             .send_request_tenders(&data.signer, bytes)
//             .unwrap()
//             .json::<TochkaApiResponse<TestSendQrPaymentResponse>>()
//     })
//     .await
//     .expect("web::block failed")
//     .map_err(anyhow::Error::from)?;

//     match res.payload {
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//         TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
//     }
// }

// #[get("/list_payments")]
// async fn list_payments(
//     data: web::Data<AppData>,
//     list_payments_req: web::Json<ListPaymentsRequest>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     let params = serde_json::to_value(&list_payments_req.0).map_err(anyhow::Error::from)?;
//     let req = serde_json::json!({
//         "jsonrpc": "2.0",
//         "id": maan_core::utils::new_uuid_v4().to_string(),
//         "method": "list_payments",
//         "params": params,
//     });
//     log::debug!("Sending request {req:#?}");
//     let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let res = web::block(move || {
//         let res = data.maan_client.send_request(&data.signer, bytes).unwrap();
//         let resp_json = res
//             .json::<serde_json::Value>()
//             .map_err(anyhow::Error::from)?;
//         log::debug!("List payment received this response {resp_json:#?}");

//         serde_json::from_value::<TochkaApiResponse<ListPaymentsResponse>>(resp_json)
//             .map_err(anyhow::Error::from)
//     })
//     .await
//     .expect("web::block failed")?;

//     match res.payload {
//         TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//     }
// }

// #[get("/get_payment")]
// async fn get_payment(
//     data: web::Data<AppData>,
//     get_payment_req: web::Json<GetPaymentRequest>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     let params = serde_json::to_value(&get_payment_req.0).map_err(anyhow::Error::from)?;
//     let req = serde_json::json!({
//         "jsonrpc": "2.0",
//         "id": maan_core::utils::new_uuid_v4().to_string(),
//         "method": "get_payment",
//         "params": params,
//     });
//     log::debug!("Sending request {req:#?}");
//     let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let res = web::block(move || {
//         let res = data.maan_client.send_request(&data.signer, bytes).unwrap();
//         let resp_json = res
//             .json::<serde_json::Value>()
//             .map_err(anyhow::Error::from)?;
//         log::debug!("Get payment received this response {resp_json:#?}");

//         serde_json::from_value::<TochkaApiResponse<GetPaymentResponseIO>>(resp_json)
//             .map_err(anyhow::Error::from)
//     })
//     .await
//     .expect("web::block failed")?;

//     match res.payload {
//         TochkaApiResponsePayload::Result { result } => {
//             Ok(HttpResponse::Ok().json(result.into_inner()))
//         }
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//     }
// }

/*
PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPG5vdGU+Cjx0bz5Ub3ZlIOmbu+e0hSDQmtCw0LrQvtC5LdGC0L4g0LrQuNGA0LjQu9C70LjRh9C10YHQutC40Lkg0YLQtdC60YHRgjwvdG8+Cjxmcm9tPkphbmk8L2Zyb20+CjxoZWFkaW5nPlJlbWluZGVyPC9oZWFkaW5nPgo8Ym9keT5Eb24ndCBmb3JnZXQgbWUgdGhpcyB3ZWVrZW5kITwvYm9keT4KPC9ub3RlPiI=
*/

// #[derive(Debug, Serialize, Deserialize)]
// struct UploadDocumentBeneficiaryRequest {
//     b64_document: String,
//     beneficiary_id: String,
//     document_number: String,
//     document_date: String,
//     content_type: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct UploadDocumentBeneficiaryResponse {
//     document_id: String,
// }

// #[post("/upload_document_beneficiary")]
// async fn upload_document_beneficiary(
//     data: web::Data<AppData>,
//     upload_document_req: web::Json<UploadDocumentBeneficiaryRequest>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     let UploadDocumentBeneficiaryRequest {
//         b64_document,
//         beneficiary_id,
//         document_number,
//         document_date,
//         content_type,
//     } = upload_document_req.0;
//     let res = web::block(move || {
//         let resp = data
//             .maan_client
//             .upload_document_beneficiary(
//                 &data.signer,
//                 b64_document,
//                 beneficiary_id,
//                 document_number,
//                 document_date,
//                 content_type,
//             )
//             .unwrap();
//         let resp_json = resp.json::<serde_json::Value>()?;
//         log::debug!("Upload document beneficiary resp - {resp_json:#?}");

//         serde_json::from_value::<UploadDocumentDealResponse>(resp_json).map_err(anyhow::Error::from)
//     })
//     .await
//     .unwrap()
//     .map_err(anyhow::Error::from)?;

//     Ok(HttpResponse::Ok().json(res))
// }

// #[get("/get_document")]
// async fn get_document(
//     data: web::Data<AppData>,
//     get_document_req: web::Json<GetDocumentRequest>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     let params = serde_json::to_value(&get_document_req.0).map_err(anyhow::Error::from)?;
//     let req = serde_json::json!({
//         "jsonrpc": "2.0",
//         "id": maan_core::utils::new_uuid_v4().to_string(),
//         "method": "get_document",
//         "params": params,
//     });
//     log::debug!("Sending request {req:#?}");
//     let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let res = web::block(move || {
//         data.maan_client
//             .send_request(&data.signer, bytes)
//             .unwrap()
//             .json::<TochkaApiResponse<GetDocumentResponseIO>>()
//     })
//     .await
//     .expect("web::block failed")
//     .map_err(anyhow::Error::from)?;

//     match res.payload {
//         TochkaApiResponsePayload::Result { result } => {
//             Ok(HttpResponse::Ok().json(result.into_inner()))
//         }
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct CharityProject {
    id: String,
    name: String,
    // TODO should be limited string
    description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NewCharityProjectRequest {
    beneficiary_id: String,
    name: String,
    // TODO should be limited string
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct NewCharityProjectResponse {
    id: String,
}

#[post("/create_charity_project")]
async fn create_charity_project(
    data: web::Data<AppData>,
    charity_project_req: web::Json<NewCharityProjectRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    // TODO change error

    // Check unique name
    let maybe_projects = data
        .store
        .lock()
        .await
        .get_beneficiary_charity_projects(&charity_project_req.beneficiary_id)
        .await?;
    if let Some(projects) = maybe_projects {
        for project_id in projects {
            let Some(project) = data
                .store
                .lock()
                .await
                .get_charity_project(&project_id)
                .await?
            else {
                continue;
            };

            if project.name == charity_project_req.name {
                return Ok(HttpResponse::InternalServerError()
                    .json("Project with the same name already exists"));
            }
        }
    }

    let params = CreateVirtualAccountRequest {
        beneficiary_id: charity_project_req.beneficiary_id.clone(),
        virtual_account_type: None,
    };
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": maan_core::utils::new_uuid_v4().to_string(),
        "method": "create_virtual_account",
        "params": params,
    });

    log::debug!("Sending request {req:#?}");

    let req_bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    let resp = data
        .maan_client
        .send_request(&data.signer, req_bytes)
        .await
        .context("failed sending the create_virtual_account request from new_charity_project")
        .map_err(anyhow::Error::from)?;
    let json_resp = resp
        .json::<TochkaApiResponse<CreateVirtualAccountResponse>>()
        .await
        .context("failed decoding the create_virtual_account response for new_charity_project")
        .map_err(anyhow::Error::from)?;

    match json_resp.payload {
        TochkaApiResponsePayload::Result { result } => {
            let NewCharityProjectRequest {
                beneficiary_id,
                name,
                description,
            } = charity_project_req.0;
            let charity_project = CharityProject {
                id: result.virtual_account.clone(),
                name,
                description,
            };
            data.store
                .lock()
                .await
                .store_charity_project(beneficiary_id, charity_project)
                .await
                .context("failed to store charity project to db")?;

            Ok(HttpResponse::Ok().json(NewCharityProjectResponse {
                id: result.virtual_account,
            }))
        }
        TochkaApiResponsePayload::Error { error } => {
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ListAllCharityProjectsResponse {
    projects: Vec<CharityProject>,
}

#[get("/list_all_projects")]
async fn list_all_projects(
    data: web::Data<AppData>,
) -> Result<impl Responder, AnyhowResponseError> {
    let projects = data
        .store
        .lock()
        .await
        .get_all_charity_projects()
        .await
        .context("failed to get all charity projects from db")?;

    Ok(HttpResponse::Ok().json(ListAllCharityProjectsResponse { projects }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ListCharityProjectsRequest {
    beneficiary_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ListCharityProjectsResponse {
    projects: Vec<CharityProject>,
}

// TODO [IMPORTANT] test case when DB lost beneficiary ids
#[get("/list_beneficiary_projects")]
async fn list_beneficiary_projects(
    data: web::Data<AppData>,
    list_charity_projects_req: web::Json<ListCharityProjectsRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let project_ids = data
        .store
        .lock()
        .await
        .get_beneficiary_charity_projects(&list_charity_projects_req.beneficiary_id)
        .await
        .context("failed to get charity projects from db")?
        .unwrap_or_default();

    let mut projects = Vec::with_capacity(project_ids.len());
    for id in project_ids {
        let maybe_project = data
            .store
            .lock()
            .await
            .get_charity_project(&id)
            .await
            .context("failed to get charity project from db")?;

        match maybe_project {
            Some(project) => projects.push(project),
            None => log::error!("Project with id {} not found in db, but must be", id),
        }
    }

    Ok(HttpResponse::Ok().json(ListCharityProjectsResponse { projects }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GetCharityProjectRequest {
    project_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GetCharityProjectResponse {
    project: CharityProject,
}

#[get("/get_charity_project")]
async fn get_charity_project(
    data: web::Data<AppData>,
    get_charity_project_req: web::Json<GetCharityProjectRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let GetCharityProjectRequest { project_id } = get_charity_project_req.0;
    let maybe_project = data
        .store
        .lock()
        .await
        .get_charity_project(&project_id)
        .await
        .context("failed to get charity project from db")?;

    match maybe_project {
        Some(project) => Ok(HttpResponse::Ok().json(GetCharityProjectResponse { project })),
        None => {
            let req = GetVirtualAccountRequest {
                virtual_account: project_id.clone(),
            };
            let params = serde_json::to_value(&req).map_err(anyhow::Error::from)?;
            let req = serde_json::json!({
                "jsonrpc": "2.0",
                "id": maan_core::utils::new_uuid_v4().to_string(),
                "method": "get_virtual_account",
                "params": params,
            });

            log::debug!("Sending request {req:#?}");

            let req_bytes = serde_json::to_vec(&req).expect("failed to serialize request");
            let resp = data
                .maan_client
                .send_request(&data.signer, req_bytes)
                .await
                .context("failed sending the get_charity_project request")?;
            let resp_json = resp
                .json::<serde_json::Value>()
                .await
                .context(
                    "failed decoding the get_virtual_account response to raw serde_json::Value",
                )
                .map_err(anyhow::Error::from)?;
            log::debug!("Get virtual account response - {resp_json:#?}");

            let resp_json =
                serde_json::from_value::<TochkaApiResponse<GetVirtualAccountResponseIO>>(resp_json)
                    .context("failed decoding response to `GetVirtualAccountResponseIO`")
                    .map_err(anyhow::Error::from)?;

            if resp_json.payload.is_ok() {
                log::warn!(
                    "Project with id {project_id} not found in maan db, but found in Tochka db"
                );
            }

            // TODO handle that properly?
            Ok(HttpResponse::InternalServerError()
                .json("Project with the same name already exists"))
        }
    }
}

// #[get("/get_virtual_account")]
// async fn get_virtual_account(
//     data: web::Data<AppData>,
//     get_virtual_acc_req: web::Json<GetVirtualAccountRequest>,
// ) -> Result<impl Responder, AnyhowResponseError> {
// let params = serde_json::to_value(&get_virtual_acc_req.0).map_err(anyhow::Error::from)?;
// let req = serde_json::json!({
//     "jsonrpc": "2.0",
//     "id": maan_core::utils::new_uuid_v4().to_string(),
//     "method": "get_virtual_account",
//     "params": params,
// });
// log::debug!("Sending request {req:#?}");
// let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
// let res = web::block(move || {
//     let res = data.maan_client.send_request(&data.signer, bytes).unwrap();
//     let resp_json = res
//         .json::<serde_json::Value>()
//         .map_err(anyhow::Error::from)?;
//     log::debug!("Get virtual account - {resp_json:#?}");

//     serde_json::from_value::<TochkaApiResponse<GetVirtualAccountResponseIO>>(resp_json)
//         .map_err(anyhow::Error::from)
// })
// .await
// .expect("web::block failed")?;

// match res.payload {
//     TochkaApiResponsePayload::Result { result } => {
//         Ok(HttpResponse::Ok().json(result.into_inner()))
//     }
//     TochkaApiResponsePayload::Error { error } => {
//         Ok(HttpResponse::InternalServerError().json(error))
//     }
// }
// }

// #[get("/list_virtual_account")]
// async fn list_virtual_account(
//     data: web::Data<AppData>,
//     list_virtual_account_req: web::Json<ListVirtualAccountRequest>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     let params = serde_json::to_value(&list_virtual_account_req.0).map_err(anyhow::Error::from)?;
//     let req = serde_json::json!({
//         "jsonrpc": "2.0",
//         "id": maan_core::utils::new_uuid_v4().to_string(),
//         "method": "list_virtual_account",
//         "params": params,
//     });
//     log::debug!("Sending request {req:#?}");
//     let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let res = web::block(move || {
//         data.maan_client
//             .send_request(&data.signer, bytes)
//             .unwrap()
//             .json::<TochkaApiResponse<ListVirtualAccountResponse>>()
//     })
//     .await
//     .expect("web::block failed")
//     .map_err(anyhow::Error::from)?;

//     match res.payload {
//         TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//     }
// }

// #[post("/identify_payment")]
// async fn identify_payment(
//     data: web::Data<AppData>,
//     identify_payment_req: web::Json<IdentificationPaymentRequest>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     let params = serde_json::to_value(&identify_payment_req.0).map_err(anyhow::Error::from)?;
//     let req = serde_json::json!({
//         "jsonrpc": "2.0",
//         "id": maan_core::utils::new_uuid_v4().to_string(),
//         "method": "identification_payment",
//         "params": params,
//     });
//     log::debug!("Sending request {req:#?}");
//     let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let res = web::block(move || {
//         let resp = data.maan_client.send_request(&data.signer, bytes).unwrap();

//         let resp_json = resp
//             .json::<serde_json::Value>()
//             .map_err(anyhow::Error::from)?;
//         log::debug!("Identify payment resp - {resp_json:#?}");

//         serde_json::from_value::<TochkaApiResponse<IdentificationPaymentResponse>>(resp_json)
//             .map_err(anyhow::Error::from)
//     })
//     .await
//     .expect("web::block failed")?;

//     match res.payload {
//         TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//     }
// }

// #[post("/create_deal")]
// async fn create_deal(
//     data: web::Data<AppData>,
//     create_deal_req: web::Json<CreateDealRequest>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     let params = serde_json::to_value(&create_deal_req.0).map_err(anyhow::Error::from)?;
//     let req = serde_json::json!({
//         "jsonrpc": "2.0",
//         "id": maan_core::utils::new_uuid_v4().to_string(),
//         "method": "create_deal",
//         "params": params,
//     });
//     log::debug!("Sending request {req:#?}");
//     let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let res = web::block(move || {
//         data.maan_client
//             .send_request(&data.signer, bytes)
//             .unwrap()
//             .json::<TochkaApiResponse<CreateDealResponse>>()
//     })
//     .await
//     .expect("web::block failed")
//     .map_err(anyhow::Error::from)?;

//     match res.payload {
//         TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//     }
// }

// #[post("/update_deal")]
// async fn update_deal(
//     data: web::Data<AppData>,
//     update_deal_req: web::Json<UpdateDealRequest>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     let params = serde_json::to_value(&update_deal_req.0).map_err(anyhow::Error::from)?;
//     let req = serde_json::json!({
//         "jsonrpc": "2.0",
//         "id": maan_core::utils::new_uuid_v4().to_string(),
//         "method": "update_deal",
//         "params": params,
//     });
//     log::debug!("Sending request {req:#?}");
//     let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let res = web::block(move || {
//         let resp = data.maan_client.send_request(&data.signer, bytes).unwrap();

//         let resp_json = resp
//             .json::<serde_json::Value>()
//             .map_err(anyhow::Error::from)?;
//         log::debug!("Update deal resp - {resp_json:#?}");

//         serde_json::from_value::<TochkaApiResponse<UpdateDealResponse>>(resp_json)
//             .map_err(anyhow::Error::from)
//     })
//     .await
//     .expect("web::block failed")
//     .map_err(anyhow::Error::from)?;

//     match res.payload {
//         TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct UploadDocumentDealRequest {
//     b64_document: String,
//     beneficiary_id: String,
//     deal_id: String,
//     document_number: String,
//     document_date: String,
//     content_type: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct UploadDocumentDealResponse {
//     document_id: String,
// }

// #[post("/upload_document_deal")]
// async fn upload_document_deal(
//     data: web::Data<AppData>,
//     upload_document_deal_req: web::Json<UploadDocumentDealRequest>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     let UploadDocumentDealRequest {
//         b64_document,
//         beneficiary_id,
//         deal_id,
//         document_number,
//         document_date,
//         content_type,
//     } = upload_document_deal_req.0;
//     let res = web::block(move || {
//         let resp = data
//             .maan_client
//             .upload_document_deal(
//                 &data.signer,
//                 b64_document,
//                 beneficiary_id,
//                 deal_id,
//                 document_number,
//                 document_date,
//                 content_type,
//             )
//             .unwrap();
//         let resp_json = resp.json::<serde_json::Value>()?;
//         log::debug!("Upload document deal resp - {resp_json:#?}");

//         serde_json::from_value::<UploadDocumentDealResponse>(resp_json).map_err(anyhow::Error::from)
//     })
//     .await
//     .unwrap()
//     .map_err(anyhow::Error::from)?;

//     Ok(HttpResponse::Ok().json(res))
// }

// // TODO FIX IT!
// #[get("/get_deal")]
// async fn get_deal(
//     data: web::Data<AppData>,
//     get_deal_req: web::Json<GetDealRequest>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     let params = serde_json::to_value(&get_deal_req.0).map_err(anyhow::Error::from)?;
//     let req = serde_json::json!({
//         "jsonrpc": "2.0",
//         "id": maan_core::utils::new_uuid_v4().to_string(),
//         "method": "get_deal",
//         "params": params,
//     });
//     log::debug!("Sending request {req:#?}");
//     let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let res = web::block(move || {
//         let resp = data.maan_client.send_request(&data.signer, bytes).unwrap();

//         let resp_json = resp
//             .json::<serde_json::Value>()
//             .map_err(anyhow::Error::from)?;
//         log::debug!("Deal info {resp_json:#?}");

//         serde_json::from_value::<TochkaApiResponse<GetDealResponse>>(resp_json)
//             .map_err(anyhow::Error::from)
//     })
//     .await
//     .expect("web::block failed")
//     .map_err(anyhow::Error::from)?;

//     match res.payload {
//         TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//     }
// }

// #[get("/list_deals")]
// async fn list_deals(
//     data: web::Data<AppData>,
//     list_deals_req: web::Json<ListDealsRequest>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     let params = serde_json::to_value(&list_deals_req.0).map_err(anyhow::Error::from)?;
//     let req = serde_json::json!({
//         "jsonrpc": "2.0",
//         "id": maan_core::utils::new_uuid_v4().to_string(),
//         "method": "list_deals",
//         "params": params,
//     });
//     log::debug!("Sending request {req:#?}");
//     let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let res = web::block(move || {
//         let resp = data.maan_client.send_request(&data.signer, bytes).unwrap();

//         let resp_json = resp
//             .json::<serde_json::Value>()
//             .map_err(anyhow::Error::from)?;
//         log::debug!("List deals info {resp_json:#?}");

//         serde_json::from_value::<TochkaApiResponse<ListDealsReponse>>(resp_json)
//             .map_err(anyhow::Error::from)
//     })
//     .await
//     .expect("web::block failed")
//     .map_err(anyhow::Error::from)?;

//     match res.payload {
//         TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//     }
// }

// #[post("/execute_deal")]
// async fn execute_deal(
//     data: web::Data<AppData>,
//     deal_id_req: web::Json<ExecuteDealRequest>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     let params = serde_json::to_value(&deal_id_req.0).map_err(anyhow::Error::from)?;
//     let req = serde_json::json!({
//         "jsonrpc": "2.0",
//         "id": maan_core::utils::new_uuid_v4().to_string(),
//         "method": "execute_deal",
//         "params": params,
//     });
//     log::debug!("Sending request {req:#?}");
//     let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let res = web::block(move || {
//         data.maan_client
//             .send_request(&data.signer, bytes)
//             .unwrap()
//             .json::<TochkaApiResponse<ExecuteDealResponse>>()
//     })
//     .await
//     .expect("web::block failed")
//     .map_err(anyhow::Error::from)?;

//     match res.payload {
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//         TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
//     }
// }

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
            .service(get_beneficiary)
            .service(create_charity_project)
            .service(list_all_projects)
            .service(list_beneficiary_projects)
            .service(get_charity_project)
        // .service(generate_sbp_qrcode)
        // .service(list_payments)
        // .service(upload_document_beneficiary)
        // .service(identify_payment)
        // .service(upload_document_deal)
        // .service(execute_deal)
        // .service(test_send_qr_payment)
        // .service(test_send_payment)
        // .service(get_payment)
        // .service(list_virtual_account)
        // .service(get_virtual_account)
        // .service(get_document)
        // .service(create_deal)
        // .service(get_deal)
        // .service(list_deals)
        // .service(get_beneficiary)
        // .service(update_deal)
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
    use actix_web::{
        dev::{HttpServiceFactory, Service, ServiceRequest, ServiceResponse},
        test, Error,
    };
    use anyhow::Context;
    use maan_core::tochka::{
        create_beneficiary::BeneficiaryData, list_beneficiary::ListBeneficiaryFilters, TochkaError,
    };
    use serde::de::DeserializeOwned;
    use std::{any, env, future::Future, pin::Pin};

    const CONFIG_PATH: &str = "./.test_config.toml";

    struct MaanTestService {
        data: web::Data<AppData>,
    }

    impl MaanTestService {
        fn new() -> anyhow::Result<Self> {
            let args = {
                let config_content = fs::read_to_string(PathBuf::from(CONFIG_PATH))
                    .context("can't read config file")?;
                toml::from_str::<Args>(&config_content).context("failed to parse into `Params`")?
            };

            let signer = {
                let private_key_string = fs::read_to_string(args.private_key_path)
                    .context("failed to read private key file")?;
                Signer::new(private_key_string).context("failed to create signer")?
            };
            let maan_client =
                MaanClient::new(args.sign_system, args.sign_thumbprint, args.endpoint);
            let store = {
                let store: Box<dyn Store> = Box::new(InMemoryStore::new());
                Mutex::new(store)
            };

            let data = web::Data::new(AppData {
                store,
                maan_client,
                signer,
            });

            Ok(Self { data })
        }

        async fn send_req<D: DeserializeOwned + 'static>(
            &self,
            req: impl Serialize + Debug + Clone,
            path: &'static str,
        ) -> MaanWebTestResponse<D> {
            // Create an app
            let (app, is_get) = async {
                let app_inner = App::new().app_data(self.data.clone());

                let (app_inner, is_get) = match path {
                    "/create_beneficiary_ul" => (app_inner.service(create_beneficiary_ul), false),
                    "/list_beneficiary" => (app_inner.service(list_beneficiary), true),
                    "/create_charity_project" => (app_inner.service(create_charity_project), false),
                    "/list_beneficiary_projects" => {
                        (app_inner.service(list_beneficiary_projects), true)
                    }
                    "/get_charity_project" => (app_inner.service(get_charity_project), true),
                    "/get_beneficiary" => (app_inner.service(get_beneficiary), true),
                    "/list_all_projects" => (app_inner.service(list_all_projects), true),
                    _ => panic!("Unknown service"),
                };

                (test::init_service(app_inner).await, is_get)
            }
            .await;

            log::debug!("Sending request {req:#?}");

            // Create a requests
            let actix_req = is_get
                .then_some(test::TestRequest::get())
                .unwrap_or(test::TestRequest::post())
                .uri(path)
                .set_json(req.clone())
                .to_request();
            let resp = test::call_service(&app, actix_req).await;

            // Deserialize response
            let resp = test::read_body_json(resp).await;

            resp
        }

        fn data(&self) -> &web::Data<AppData> {
            &self.data
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    enum MaanWebTestResponse<T> {
        Error(TochkaError),
        Ok(T),
    }

    impl<T> MaanWebTestResponse<T> {
        fn is_ok(&self) -> bool {
            matches!(self, MaanWebTestResponse::Ok(_))
        }

        fn is_err(&self) -> bool {
            matches!(self, MaanWebTestResponse::Error(_))
        }

        fn ok(&self) -> Option<&T> {
            match self {
                MaanWebTestResponse::Ok(v) => Some(v),
                _ => None,
            }
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
    async fn test_beneficiaries() -> anyhow::Result<()> {
        let _ = env_logger::Builder::from_default_env()
            .format_module_path(false)
            .format_level(true)
            .try_init();

        // Create test service which can send requests
        let test_service = MaanTestService::new()?;

        // Create a beneficiary request
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
        let resp = test_service
            .send_req::<CreateBeneficiaryResponse>(&req_body, "/create_beneficiary_ul")
            .await;

        // Check response
        let id = match resp {
            MaanWebTestResponse::Error(tochka_error) => {
                panic!("Failed to create beneficiary. Got error: {tochka_error:#?}");
            }
            MaanWebTestResponse::Ok(v) => {
                let CreateBeneficiaryResponse::Beneficiary { id, .. } = v;

                id
            }
        };

        // Check data is actually in DB
        let maybe_beneficiary = test_service
            .data()
            .store
            .lock()
            .await
            .get_beneficiary(&id)
            .await
            .expect("failed sending request");
        assert_eq!(maybe_beneficiary, Some(beneficiary_data.clone()));

        // Get beneficiary from server
        let req = GetBeneficiaryRequest {
            beneficiary_id: id.clone(),
        };
        let resp = test_service
            .send_req::<GetBeneficiaryResponse>(&req, "/get_beneficiary")
            .await;
        let actual_resp = resp.ok().cloned().expect("failed to get beneficiary");
        assert_eq!(actual_resp.beneficiary.id, id);
        assert_eq!(
            actual_resp.beneficiary.beneficiary_data.name,
            beneficiary_data.name
        );
        assert_eq!(
            actual_resp.beneficiary.beneficiary_data.kpp,
            beneficiary_data.kpp
        );
        assert_eq!(actual_resp.beneficiary.inn, req_body.inn);

        // Sending same data must fail, as beneficiary already exists
        let resp = test_service
            .send_req::<CreateBeneficiaryResponse>(&req_body, "/create_beneficiary_ul")
            .await;
        assert!(resp.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_charity_projects() -> anyhow::Result<()> {
        let _ = env_logger::Builder::from_default_env()
            .format_module_path(false)
            .format_level(true)
            .try_init();

        // Create test service which can send requests
        let test_service = MaanTestService::new()?;

        // Get list of beneficiaries
        let req = ListBeneficiaryRequest {
            filters: ListBeneficiaryFilters {
                inn: None,
                nominal_account_code: None,
                nominal_account_bic: None,
                is_active: None,
                legal_type: None,
            },
        };
        let MaanWebTestResponse::Ok(mut resp) = test_service
            .send_req::<ListBeneficiaryResponse>(&req, "/list_beneficiary")
            .await
        else {
            return Err(anyhow::anyhow!("Failed to list beneficiaries"));
        };

        // If beneficiaries list is empty, create a new beneficiary
        let maybe_beneficiary_id = resp.beneficiaries.pop().map(|v| v.id);
        let beneficiary_id = if let Some(beneficiary_id) = maybe_beneficiary_id {
            beneficiary_id
        } else {
            // Create a beneficiary request
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

            let resp = test_service
                .send_req::<CreateBeneficiaryResponse>(&req_body, "/create_beneficiary_ul")
                .await;

            // Check response
            match resp {
                MaanWebTestResponse::Error(tochka_error) => {
                    panic!("Failed to create beneficiary. Got error: {tochka_error:#?}");
                }
                MaanWebTestResponse::Ok(v) => {
                    let CreateBeneficiaryResponse::Beneficiary { id, .. } = v;

                    id
                }
            }
        };

        // Create a couple of charity projects
        let req_proj_1 = NewCharityProjectRequest {
            beneficiary_id: beneficiary_id.clone(),
            name: "Project 1".to_string(),
            description: "Project 1 is meant for testing".to_string(),
        };
        let resp = test_service
            .send_req::<NewCharityProjectResponse>(req_proj_1.clone(), "/create_charity_project")
            .await;
        let project_1_id = resp
            .ok()
            .expect("internal error: failed to create charity project 1")
            .id
            .clone();

        let actual_charity_project_1 = test_service
            .data
            .store
            .lock()
            .await
            .get_charity_project(&project_1_id)
            .await
            .context("failed to get from db charity project 1")?;
        let expected_charity_project_1 = CharityProject {
            id: project_1_id.clone(),
            name: req_proj_1.name.clone(),
            description: req_proj_1.description.clone(),
        };
        assert_eq!(
            actual_charity_project_1,
            Some(expected_charity_project_1.clone())
        );

        let req_proj_2 = NewCharityProjectRequest {
            beneficiary_id: beneficiary_id.clone(),
            name: "Project 2".to_string(),
            description: "Project 2 is meant for testing".to_string(),
        };
        let resp = test_service
            .send_req::<NewCharityProjectResponse>(req_proj_2.clone(), "/create_charity_project")
            .await;
        let project_2_id = resp
            .ok()
            .expect("internal error: failed to create charity project 2")
            .id
            .clone();

        let actual_charity_project_2 = test_service
            .data
            .store
            .lock()
            .await
            .get_charity_project(&project_2_id)
            .await
            .context("failed to get from db charity project 2")?;
        let expected_charity_project_2 = CharityProject {
            id: project_2_id.clone(),
            name: req_proj_2.name.clone(),
            description: req_proj_2.description.clone(),
        };
        assert_eq!(
            actual_charity_project_2,
            Some(expected_charity_project_2.clone())
        );

        // Check list of projects in db contains created projects
        let expected_list_of_projects = vec![project_1_id.clone(), project_2_id.clone()];
        let actual_list_of_projects = test_service
            .data
            .store
            .lock()
            .await
            .get_beneficiary_charity_projects(&beneficiary_id)
            .await?
            .expect("internal error: charity projects for the beneficiary is empty");
        assert_eq!(actual_list_of_projects, expected_list_of_projects);

        // A request should return the same list
        let req = ListCharityProjectsRequest { beneficiary_id };
        let resp = test_service
            .send_req::<ListCharityProjectsResponse>(req, "/list_beneficiary_projects")
            .await;
        let actual_projects = resp
            .ok()
            .cloned()
            .expect("internal error: failed to list charity projects")
            .projects;
        let expected_projects = vec![
            expected_charity_project_1.clone(),
            expected_charity_project_2.clone(),
        ];
        assert_eq!(actual_projects, expected_projects);

        // Get existing project
        let req = GetCharityProjectRequest {
            project_id: project_1_id.clone(),
        };
        let resp = test_service
            .send_req::<GetCharityProjectResponse>(req, "/get_charity_project")
            .await;
        let actual_project = resp
            .ok()
            .cloned()
            .expect("internal error: failed to get charity project")
            .project;
        assert_eq!(actual_project, expected_charity_project_1);

        // Get non-existent project
        let req = GetCharityProjectRequest {
            project_id: "non-existent-project-id".to_string(),
        };
        let resp = test_service
            .send_req::<String>(req, "/get_charity_project")
            .await;
        let actual_project = resp
            .ok()
            .expect("internal error: failed to get charity project");
        assert_eq!(actual_project, "Project with the same name already exists");

        Ok(())
    }

    #[tokio::test]
    async fn test_all_charity_projects() -> anyhow::Result<()> {
        let _ = env_logger::Builder::from_default_env()
            .format_module_path(false)
            .format_level(true)
            .try_init();

        // Create test service which can send requests
        let test_service = MaanTestService::new()?;

        // Create 2 new beneficiaries
        let beneficiary_data = BeneficiaryData {
            name: "ООО \"Петруня\"".to_string(),
            kpp: "667101001".to_string(),
            ogrn: None,
            is_branch: None,
        };
        let req_body_1 = CreateBeneficiaryUlRequest {
            inn: generate_random_inn_j(),
            nominal_account_code: "40702810620000088278".to_string(),
            nominal_account_bic: "044525104".to_string(),
            beneficiary_data: beneficiary_data.clone(),
        };
        let MaanWebTestResponse::Ok(CreateBeneficiaryResponse::Beneficiary {
            id: beneficiary_id_1,
            ..
        }) = test_service
            .send_req::<CreateBeneficiaryResponse>(&req_body_1, "/create_beneficiary_ul")
            .await
        else {
            panic!("failed to create beneficiary")
        };

        let req_body_2 = CreateBeneficiaryUlRequest {
            inn: generate_random_inn_j(),
            nominal_account_code: "40702810620000088278".to_string(),
            nominal_account_bic: "044525104".to_string(),
            beneficiary_data: beneficiary_data.clone(),
        };
        let MaanWebTestResponse::Ok(CreateBeneficiaryResponse::Beneficiary {
            id: beneficiary_id_2,
            ..
        }) = test_service
            .send_req::<CreateBeneficiaryResponse>(&req_body_2, "/create_beneficiary_ul")
            .await
        else {
            panic!("failed to create beneficiary")
        };

        // Create a couple of charity projects
        let req_proj_1 = NewCharityProjectRequest {
            beneficiary_id: beneficiary_id_1.clone(),
            name: "Project 1".to_string(),
            description: "Project 1 is meant for testing".to_string(),
        };
        let resp = test_service
            .send_req::<NewCharityProjectResponse>(req_proj_1.clone(), "/create_charity_project")
            .await;
        let beneficiary_1_proj_1 = CharityProject {
            id: resp
                .ok()
                .expect("internal error: failed to create charity project 1 for beneficiary 1")
                .id
                .clone(),
            name: req_proj_1.name.clone(),
            description: req_proj_1.description.clone(),
        };

        let req_proj_2 = NewCharityProjectRequest {
            beneficiary_id: beneficiary_id_2.clone(),
            name: "Project 2".to_string(),
            description: "Project 2 is meant for testing".to_string(),
        };
        let resp = test_service
            .send_req::<NewCharityProjectResponse>(req_proj_2.clone(), "/create_charity_project")
            .await;
        let beneficiary_1_proj_2 = CharityProject {
            id: resp
                .ok()
                .expect("internal error: failed to create charity project 2 for beneficiary 1")
                .id
                .clone(),
            name: req_proj_2.name.clone(),
            description: req_proj_2.description.clone(),
        };

        let req_proj_3 = NewCharityProjectRequest {
            beneficiary_id: beneficiary_id_2.clone(),
            name: "Project 3".to_string(),
            description: "Project 3 is meant for testing".to_string(),
        };
        let resp = test_service
            .send_req::<NewCharityProjectResponse>(req_proj_3.clone(), "/create_charity_project")
            .await;
        let beneficiary_2_proj_1 = CharityProject {
            id: resp
                .ok()
                .expect("internal error: failed to create charity project 1 for beneficiary 2")
                .id
                .clone(),
            name: req_proj_3.name.clone(),
            description: req_proj_3.description.clone(),
        };

        // List all charity projects
        let resp = test_service
            .send_req::<ListAllCharityProjectsResponse>((), "/list_all_projects")
            .await;
        let actual_projects = resp
            .ok()
            .cloned()
            .expect("internal error: failed to list all charity projects")
            .projects;

        // Check that the created projects are in the list
        let expected_projects = vec![
            beneficiary_1_proj_1,
            beneficiary_1_proj_2,
            beneficiary_2_proj_1,
        ];
        assert_eq!(actual_projects, expected_projects);

        Ok(())
    }
}
