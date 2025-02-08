// TODO:
// 1. use anyhow for the error

mod db;
mod error;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Context;
use clap::Parser;
use db::{InMemoryStore, Store};
use error::AnyhowResponseError;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, path::PathBuf};
use tochka_cyclops::{tochka_io::*, RsaSigner, TochkaApiResponsePayload, TochkaCyclopsClient};
use tokio::sync::Mutex;

struct AppData {
    store: Mutex<Box<dyn Store>>,
    tochka_client: TochkaCyclopsClient<RsaSigner>,
}

// TODO input is just inn and beneficiary data
#[post("/create_beneficiary_ul")]
async fn create_beneficiary_ul(
    data: web::Data<AppData>,
    create_beneficiary: web::Json<CreateBeneficiaryUlRequest>,
) -> Result<impl Responder, AnyhowResponseError> {
    let web::Json(create_beneficiary) = create_beneficiary;
    let beneficiary_data = create_beneficiary.beneficiary_data.clone();
    let resp = data
        .tochka_client
        .create_beneficiary_ul(create_beneficiary)
        .await?;

    match resp.payload {
        TochkaApiResponsePayload::Result { result } => {
            let CreateBeneficiaryResponse::Beneficiary { id, .. } = &result;
            data.store
                .lock()
                .await
                .store_beneficiary(id.clone(), beneficiary_data)
                .await?;

            tokio::spawn(check_added_to_ms(data, id.clone()));

            Ok(HttpResponse::Ok().json(result))
        }
        TochkaApiResponsePayload::Error { error } => {
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

async fn check_added_to_ms(data: web::Data<AppData>, beneficiary_id: String) {
    todo!()
    // let AppData {
    //     store,
    //     maan_client,
    //     signer,
    // } = data.get_ref();
    // let req = GetBeneficiaryRequest {
    //     beneficiary_id: beneficiary_id.clone(),
    // };
    // let params = match serde_json::to_value(&req) {
    //     Ok(params) => params,
    //     Err(e) => {
    //         log::error!("Failed to serialize get_beneficiary request: {e}");
    //         return;
    //     }
    // };

    // let req = serde_json::json!({
    //     "jsonrpc": "2.0",
    //     "id": maan_core::utils::new_uuid_v4().to_string(),
    //     "method": "get_beneficiary",
    //     "params": params,
    // });

    // log::debug!("Sending request {req:#?}");

    // loop {
    //     let req_bytes = serde_json::to_vec(&req).expect("failed to serialize request");
    //     let resp = match maan_client.send_request(&signer, req_bytes).await {
    //         Ok(resp) => resp,
    //         Err(e) => {
    //             log::error!("Failed to send get_beneficiary request: {e}");
    //             return;
    //         }
    //     };
    //     let json_resp = match resp
    //         .json::<TochkaApiResponse<GetBeneficiaryResponse>>()
    //         .await
    //     {
    //         Ok(json_resp) => json_resp,
    //         Err(e) => {
    //             log::error!("Failed to decode get_beneficiary response: {e}");
    //             return;
    //         }
    //     };

    //     match json_resp.payload {
    //         TochkaApiResponsePayload::Result { result } if result.beneficiary.is_added_to_ms => {
    //             let res = store
    //                 .lock()
    //                 .await
    //                 .set_added_to_ms(beneficiary_id.clone())
    //                 .await;

    //             if let Err(e) = res {
    //                 // TODO must be handled properly
    //                 log::error!("Failed to set added_to_ms for beneficiary {beneficiary_id}: {e}");
    //             }

    //             return;
    //         }
    //         TochkaApiResponsePayload::Error { error } => {
    //             log::error!("Error response from get_beneficiary: {error:#?}");

    //             return;
    //         }
    //         _ => continue,
    //     }
    // }
}

// // TODO [IMPORTANT] implement get_/list_ or_update logic for list_\get_beneficiary
// #[get("/list_beneficiary")]
// async fn list_beneficiary(
//     data: web::Data<AppData>,
//     list_beneficiary_req: web::Json<ListBeneficiaryRequest>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     let params = serde_json::to_value(&list_beneficiary_req.0).map_err(anyhow::Error::from)?;
//     let req = serde_json::json!({
//         "jsonrpc": "2.0",
//         "id": maan_core::utils::new_uuid_v4().to_string(),
//         "method": "list_beneficiary",
//         "params": params,
//     });

//     log::debug!("Sending request {req:#?}");

//     let req_bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let resp = data
//         .maan_client
//         .send_request(&data.signer, req_bytes)
//         .await
//         .context("failed sending the list_beneficiary request")
//         .map_err(anyhow::Error::from)?;
//     let json_resp = resp
//         .json::<TochkaApiResponse<ListBeneficiaryResponse>>()
//         .await
//         .context("failed decoding the list_beneficiary response")
//         .map_err(anyhow::Error::from)?;

//     match json_resp.payload {
//         TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//     }
// }

// #[get("/get_beneficiary")]
// async fn get_beneficiary(
//     data: web::Data<AppData>,
//     get_beneficiary_req: web::Json<GetBeneficiaryRequest>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     let params = serde_json::to_value(&get_beneficiary_req.0).map_err(anyhow::Error::from)?;
//     let req = serde_json::json!({
//         "jsonrpc": "2.0",
//         "id": maan_core::utils::new_uuid_v4().to_string(),
//         "method": "get_beneficiary",
//         "params": params,
//     });

//     log::debug!("Sending request {req:#?}");

//     let req_bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let resp = data
//         .maan_client
//         .send_request(&data.signer, req_bytes)
//         .await
//         .context("failed sending the get_beneficiary request")?;
//     let resp_json = resp
//         .json::<serde_json::Value>()
//         .await
//         .context("failed decoding the get_beneficiary response to `serde_json::Value`")
//         .map_err(anyhow::Error::from)?;

//     log::debug!("Get beneficiary raw - response {resp_json:#?}");

//     let resp_json = serde_json::from_value::<TochkaApiResponse<GetBeneficiaryResponse>>(resp_json)
//         .context("failed decoding the get_beneficiary response into `GetBeneficiaryResponse`")
//         .map_err(anyhow::Error::from)?;

//     match resp_json.payload {
//         TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//     }
// }

// /*
// PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPG5vdGU+Cjx0bz5Ub3ZlIOmbu+e0hSDQmtCw0LrQvtC5LdGC0L4g0LrQuNGA0LjQu9C70LjRh9C10YHQutC40Lkg0YLQtdC60YHRgjwvdG8+Cjxmcm9tPkphbmk8L2Zyb20+CjxoZWFkaW5nPlJlbWluZGVyPC9oZWFkaW5nPgo8Ym9keT5Eb24ndCBmb3JnZXQgbWUgdGhpcyB3ZWVrZW5kITwvYm9keT4KPC9ub3RlPiI=
// */
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

//     // TODO - test isolated, that it returns a new document id for the same request data
//     let resp = data
//         .maan_client
//         .upload_document_beneficiary(
//             &data.signer,
//             b64_document.clone(),
//             beneficiary_id.clone(),
//             document_number,
//             document_date,
//             content_type,
//         )
//         .await
//         .context("failed sending the upload_document request")?;

//     let resp_json = resp
//         .json::<serde_json::Value>()
//         .await
//         .context("failed decoding the `upload_document` response to the `serde_json::Value`")?;

//     log::debug!("Upload document beneficiary response - {resp_json:#?}");

//     // TODO error type here differs from Tochka wrappers
//     let resp_json = serde_json::from_value::<UploadDocumentBeneficiaryResponse>(resp_json)
//         .map_err(anyhow::Error::from)
//         .context(
//             "failed decoding `upload_document` response into `UploadDocumentBeneficiaryResponse`",
//         )?;

//     data.store
//         .lock()
//         .await
//         .store_beneficiary_document(
//             beneficiary_id,
//             (resp_json.document_id.clone(), b64_document),
//         )
//         .await
//         .context("failed to store beneficiary document")?;

//     Ok(HttpResponse::Ok().json(resp_json))
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
//     let req_bytes = serde_json::to_vec(&req).expect("failed to serialize request");

//     let resp = data
//         .maan_client
//         .send_request(&data.signer, req_bytes)
//         .await
//         .context("failed to send the get_document request")?;
//     let resp_json = resp
//         .json::<TochkaApiResponse<GetDocumentResponseIO>>()
//         .await
//         .map_err(anyhow::Error::from)
//         .context("failed to decode the get_document response to `GetDocumentResponseIO`")?;

//     match resp_json.payload {
//         TochkaApiResponsePayload::Result { result } => {
//             Ok(HttpResponse::Ok().json(result.into_inner()))
//         }
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
struct CharityProject {
    id: String,
    name: String,
    // TODO should be limited string
    description: String,
    cap: u32,
    collected: u32,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct NewCharityProjectRequest {
//     beneficiary_id: String,
//     name: String,
//     // TODO should be limited string
//     description: String,
//     cap: u32,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct NewCharityProjectResponse {
//     id: String,
// }

// #[post("/create_charity_project")]
// async fn create_charity_project(
//     data: web::Data<AppData>,
//     charity_project_req: web::Json<NewCharityProjectRequest>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     // TODO change error

//     // Check unique name
//     let maybe_projects = data
//         .store
//         .lock()
//         .await
//         .get_beneficiary_charity_projects(&charity_project_req.beneficiary_id)
//         .await?;
//     if let Some(projects) = maybe_projects {
//         for project_id in projects {
//             let Some(project) = data
//                 .store
//                 .lock()
//                 .await
//                 .get_charity_project(&project_id)
//                 .await?
//             else {
//                 continue;
//             };

//             if project.name == charity_project_req.name {
//                 return Ok(HttpResponse::InternalServerError()
//                     .json("Project with the same name already exists"));
//             }
//         }
//     }

//     let params = CreateVirtualAccountRequest {
//         beneficiary_id: charity_project_req.beneficiary_id.clone(),
//         virtual_account_type: None,
//     };
//     let req = serde_json::json!({
//         "jsonrpc": "2.0",
//         "id": maan_core::utils::new_uuid_v4().to_string(),
//         "method": "create_virtual_account",
//         "params": params,
//     });

//     log::debug!("Sending request {req:#?}");

//     let req_bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let resp = data
//         .maan_client
//         .send_request(&data.signer, req_bytes)
//         .await
//         .context("failed sending the create_virtual_account request from new_charity_project")
//         .map_err(anyhow::Error::from)?;
//     let resp_json = resp
//         .json::<TochkaApiResponse<CreateVirtualAccountResponse>>()
//         .await
//         .context("failed decoding the create_virtual_account response for new_charity_project")
//         .map_err(anyhow::Error::from)?;

//     match resp_json.payload {
//         TochkaApiResponsePayload::Result { result } => {
//             let NewCharityProjectRequest {
//                 beneficiary_id,
//                 name,
//                 description,
//                 cap,
//             } = charity_project_req.0;
//             let charity_project = CharityProject {
//                 id: result.virtual_account.clone(),
//                 name,
//                 description,
//                 cap,
//                 collected: 0,
//             };
//             data.store
//                 .lock()
//                 .await
//                 .store_charity_project(beneficiary_id, charity_project)
//                 .await
//                 .context("failed to store charity project to db")?;

//             Ok(HttpResponse::Ok().json(NewCharityProjectResponse {
//                 id: result.virtual_account,
//             }))
//         }
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct ListAllCharityProjectsResponse {
//     projects: Vec<CharityProject>,
// }

// #[get("/list_all_projects")]
// async fn list_all_projects(
//     data: web::Data<AppData>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     let projects = data
//         .store
//         .lock()
//         .await
//         .get_all_charity_projects()
//         .await
//         .context("failed to get all charity projects from db")?;

//     Ok(HttpResponse::Ok().json(ListAllCharityProjectsResponse { projects }))
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct ListCharityProjectsRequest {
//     beneficiary_id: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct ListCharityProjectsResponse {
//     projects: Vec<CharityProject>,
// }

// // TODO [IMPORTANT] test case when DB lost beneficiary ids
// #[get("/list_beneficiary_projects")]
// async fn list_beneficiary_projects(
//     data: web::Data<AppData>,
//     list_charity_projects_req: web::Json<ListCharityProjectsRequest>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     let project_ids = data
//         .store
//         .lock()
//         .await
//         .get_beneficiary_charity_projects(&list_charity_projects_req.beneficiary_id)
//         .await
//         .context("failed to get charity projects from db")?
//         .unwrap_or_default();

//     let mut projects = Vec::with_capacity(project_ids.len());
//     for id in project_ids {
//         let maybe_project = data
//             .store
//             .lock()
//             .await
//             .get_charity_project(&id)
//             .await
//             .context("failed to get charity project from db")?;

//         match maybe_project {
//             Some(project) => projects.push(project),
//             None => log::error!("Project with id {} not found in db, but must be", id),
//         }
//     }

//     Ok(HttpResponse::Ok().json(ListCharityProjectsResponse { projects }))
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct GetCharityProjectRequest {
//     project_id: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct GetCharityProjectResponse {
//     project: CharityProject,
// }

// #[get("/get_charity_project")]
// async fn get_charity_project(
//     data: web::Data<AppData>,
//     get_charity_project_req: web::Json<GetCharityProjectRequest>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     let GetCharityProjectRequest { project_id } = get_charity_project_req.0;
//     let maybe_project = data
//         .store
//         .lock()
//         .await
//         .get_charity_project(&project_id)
//         .await
//         .context("failed to get charity project from db")?;

//     match maybe_project {
//         Some(project) => Ok(HttpResponse::Ok().json(GetCharityProjectResponse { project })),
//         None => {
//             let req = GetVirtualAccountRequest {
//                 virtual_account: project_id.clone(),
//             };
//             let params = serde_json::to_value(&req).map_err(anyhow::Error::from)?;
//             let req = serde_json::json!({
//                 "jsonrpc": "2.0",
//                 "id": maan_core::utils::new_uuid_v4().to_string(),
//                 "method": "get_virtual_account",
//                 "params": params,
//             });

//             log::debug!("Sending request {req:#?}");

//             let req_bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//             let resp = data
//                 .maan_client
//                 .send_request(&data.signer, req_bytes)
//                 .await
//                 .context("failed sending the get_charity_project request")?;
//             let resp_json = resp
//                 .json::<serde_json::Value>()
//                 .await
//                 .context(
//                     "failed decoding the get_virtual_account response to raw serde_json::Value",
//                 )
//                 .map_err(anyhow::Error::from)?;
//             log::debug!("Get virtual account response - {resp_json:#?}");

//             let resp =
//                 serde_json::from_value::<TochkaApiResponse<GetVirtualAccountResponseIO>>(resp_json)
//                     .context("failed decoding response to `GetVirtualAccountResponseIO`")
//                     .map_err(anyhow::Error::from)?;

//             if resp.payload.is_ok() {
//                 log::warn!(
//                     "Project with id {project_id} not found in maan db, but found in Tochka db"
//                 );
//             }

//             // TODO handle that properly?
//             Ok(HttpResponse::InternalServerError()
//                 .json("Project with the same name already exists"))
//         }
//     }
// }

// // TODO ADD TO DB
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

//     let req_bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let resp = data
//         .maan_client
//         .send_request(&data.signer, req_bytes)
//         .await
//         .context("failed to send create_deal req")
//         .map_err(anyhow::Error::from)?;

//     let resp_json = resp
//         .json::<TochkaApiResponse<CreateDealResponse>>()
//         .await
//         .context("failed to decode response to `CreateDealResponse`")?;

//     match resp_json.payload {
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

// // TODO ADD TO DB DATA
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

//     // TODO [sab] check deal exists or test that tochka API will
//     // always return error when deal doesn't exist

//     let resp = data
//         .maan_client
//         .upload_document_deal(
//             &data.signer,
//             b64_document,
//             beneficiary_id,
//             deal_id,
//             document_number,
//             document_date,
//             content_type,
//         )
//         .await
//         .context("failed to send upload_document_deal request")?;

//     let resp_json = resp
//         .json::<serde_json::Value>()
//         .await
//         .map_err(anyhow::Error::from)
//         .context("failed to decode upload_document_deal response")?;

//     log::debug!("Upload document deal response - {resp_json:#?}");

//     let resp = serde_json::from_value::<UploadDocumentDealResponse>(resp_json)
//         .map_err(anyhow::Error::from)
//         .context("failed to decode response to `UploadDocumentDealResponse`")?;

//     Ok(HttpResponse::Ok().json(resp))
// }

// // TODO ADD TO DB DATA
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

//     let req_bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let resp = data
//         .maan_client
//         .send_request(&data.signer, req_bytes)
//         .await
//         .context("failed to send update_deal request")?;

//     let resp_json = resp
//         .json::<serde_json::Value>()
//         .await
//         .map_err(anyhow::Error::from)
//         .context("failed to decode response")?;

//     log::debug!("Update deal response - {resp_json:#?}");

//     let resp = serde_json::from_value::<TochkaApiResponse<UpdateDealResponse>>(resp_json)
//         .map_err(anyhow::Error::from)
//         .context("failed to decode response to `UpdateDealResponse`")?;

//     match resp.payload {
//         TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//     }
// }

// // TODO ADD TO DB DATA
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

//     let req_bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let resp = data.maan_client
//         .send_request(&data.signer, req_bytes)
//         .await
//         .context("failed to send list_deals")?;

//     let resp_json = resp
//         .json::<serde_json::Value>()
//         .await
//         .map_err(anyhow::Error::from)
//         .context("failed to decode response")?;

//     log::debug!("List deals info {resp_json:#?}");

//     let resp = serde_json::from_value::<TochkaApiResponse<ListDealsResponse>>(resp_json)
//         .map_err(anyhow::Error::from)
//         .context("failed to decode response to `ListDealsResponse`")?;

//     match resp.payload {
//         TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//     }
// }

// // TODO ADD TO DB DATA
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

//     let req_bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let resp = data.maan_client
//         .send_request(&data.signer, req_bytes)
//         .await
//         .context("failed to send get_deal request")?;

//     let resp_json = resp
//         .json::<serde_json::Value>()
//         .await
//         .map_err(anyhow::Error::from)
//         .context("failed to decode response")?;

//     log::debug!("Deal info response: {resp_json:#?}");

//     let resp = serde_json::from_value::<TochkaApiResponse<GetDealResponse>>(resp_json)
//         .map_err(anyhow::Error::from)
//         .context("failed to decode response to `GetDealResponse`")?;

//     match resp.payload {
//         TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct DonateRequest {
//     donator: String,
//     project: String,
//     amount: u32,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct DonateResponse {
//     qr_code: String,
// }

// #[post("/donate")]
// async fn donate(
//     data: web::Data<AppData>,
//     json: web::Json<DonateRequest>,
// ) -> Result<impl Responder, AnyhowResponseError> {
//     let DonateRequest { donator, project, amount } = json.0;
//     // TODO: IS IT TEMPORARY OR REMAINS ALWAYS
//     let generate_sbp_req = GenerateSbpQrCodeRequest {
//         amount: amount,
//         // format!("donation_{project}_{donator}")
//         purpose: "тест".to_string(),
//         // TODO change constants
//         nominal_account_code: "40702810620000088278".to_string(),
//         nominal_account_bic: "044525104".to_string(),
//         height: 400,
//         width: 400,
//     };

//     let params = serde_json::to_value(&generate_sbp_req).map_err(anyhow::Error::from)?;
//     let req = serde_json::json!({
//         "jsonrpc": "2.0",
//         "id": maan_core::utils::new_uuid_v4().to_string(),
//         "method": "generate_sbp_qrcode",
//         "params": params,
//     });

//     log::debug!("Sending request {req:#?}");

//     let req_bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let resp = data.maan_client
//         .send_request(&data.signer, req_bytes)
//         .await
//         .context("failed to send generate_sbp_qrcode request")?;
//     let resp_json = resp
//         .json::<TochkaApiResponse<GenerateSbpQrCodeResponseIO>>()
//         .await
//         .context("failed to decode response to `GenerateSbpQrCodeResponseIO`")?;

//     match resp_json.payload {
//         TochkaApiResponsePayload::Result { result } => {
//             let res = result.into_inner();
//             let id = res.id;
//             data.store
//                 .lock()
//                 .await
//                 .store_donation_data(id.clone(), project, amount)
//                 .await?;

//             tokio::spawn(identify_payment(data, id.clone()));

//             Ok(HttpResponse::Ok().json(DonateResponse { qr_code: id }))
//         },
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//     }
// }

// // TODO change for the real env (identify payments)
// async fn identify_payment(data: web::Data<AppData>, qr_code_id: String,) -> anyhow::Result<()> {
//     let raw_req = ListPaymentsRequest {
//         filters: ListPaymentsFilters {
//             c2b_qr_code_id: Some(qr_code_id.clone()),
//             identify: Some(false),
//             incoming: Some(true),
//             payment_type: None,
//             account: None,
//             bic: None,
//         },
//     };

//     log::warn!("[SPAWN]: Identify payment request - {raw_req:#?}");
//     loop {
//         // List payments
//         let params = serde_json::to_value(&raw_req).map_err(anyhow::Error::from)?;
//         let req = serde_json::json!({
//             "jsonrpc": "2.0",
//             "id": maan_core::utils::new_uuid_v4().to_string(),
//             "method": "list_payments",
//             "params": params,
//         });

//         log::debug!("Sending request {req:#?}");

//         let req_bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//         let resp = data.maan_client
//             .send_request(&data.signer, req_bytes)
//             .await
//             .context("failed to send list_payments request")?;
//         let resp_json = resp
//             .json::<TochkaApiResponse<ListPaymentsResponse>>()
//             .await
//             .context("failed to decode response to `ListPaymentsResponse`")?;

//         match resp_json.payload {
//             TochkaApiResponsePayload::Result { result } => {
//                 let payments = result.payments;
//                 if payments.is_empty() {
//                     continue;
//                 }

//                 // Identify if found
//                 let payment_id = payments[0].clone();
//                 let (project_id, amount) = data.store
//                     .lock()
//                     .await
//                     .get_donation_data(&qr_code_id)
//                     .await?
//                     .expect("called only for existing payments");

//                 let raw_req = IdentificationPaymentRequest {
//                     payment_id,
//                     owners: vec![
//                         PaymentOwner {
//                             amount,
//                             virtual_account: project_id.clone(),
//                         }
//                     ],
//                 };
//                 let params = serde_json::to_value(&raw_req).map_err(anyhow::Error::from)?;
//                 let req = serde_json::json!({
//                     "jsonrpc": "2.0",
//                     "id": maan_core::utils::new_uuid_v4().to_string(),
//                     "method": "identify_payment",
//                     "params": params,
//                 });

//                 log::debug!("Sending request {req:#?}");

//                 let req_bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//                 let resp = data.maan_client
//                     .send_request(&data.signer, req_bytes)
//                     .await
//                     .context("failed to send identify_payment request")?;
//                 let resp_json = resp
//                     .json::<serde_json::Value>()
//                     .await
//                     .context("failed to decode response")?;

//                 log::debug!("Identify payment response - {resp_json:#?}");

//                 let resp_json = serde_json::from_value::<TochkaApiResponse<IdentificationPaymentResponse>>(resp_json)
//                     .context("failed to decode response to `IdentificationPaymentResponse`")
//                     .map_err(anyhow::Error::from)?;

//                 log::debug!("Identify payment response - {resp_json:#?}");
//                 if resp_json.payload.is_ok() {
//                     data.store
//                         .lock()
//                         .await
//                         .increase_collected_by(&project_id, amount)
//                         .await?;
//                 }
//             },
//             TochkaApiResponsePayload::Error { error } => {
//                 log::debug!("{error:#?}");
//                 return Ok(())
//             }
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

//     let req_bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let resp = data.maan_client
//         .send_request(&data.signer, req_bytes)
//         .await
//         .context("failed to send execute_deal request")?;
//     let resp_json = resp
//         .json::<TochkaApiResponse<ExecuteDealResponse>>()
//         .await
//         .context("failed to decode response")?;

//     match resp_json.payload {
//         TochkaApiResponsePayload::Error { error } => {
//             Ok(HttpResponse::InternalServerError().json(error))
//         }
//         TochkaApiResponsePayload::Result { result } => Ok(HttpResponse::Ok().json(result)),
//     }
// }

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
//     let resp = data.maan_client
//         .send_request(&data.signer, bytes)
//         .await?
//         .json::<TochkaApiResponse<GenerateSbpQrCodeResponseIO>>()
//         .await
//         .map_err(anyhow::Error::from)?;

//     match resp.payload {
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

//     let req_bytes = serde_json::to_vec(&req).expect("failed to serialize request");
//     let resp_json = data
//         .tochka_client
//         .send_request_tenders(&data.signer, req_bytes)
//         .await?
//         .json::<TochkaApiResponse<TestSendQrPaymentResponse>>()
//         .await
//         .map_err(anyhow::Error::from)?;

//     match resp_json.payload {
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

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let _ = env_logger::Builder::from_default_env()
        .format_module_path(false)
        .format_level(true)
        .try_init();

    let signer = RsaSigner::new(args.private_key_path).context("failed to create rsa signer")?;
    let tochka_client = TochkaCyclopsClient::new(
        args.sign_system,
        args.sign_thumbprint,
        args.endpoint,
        signer,
    );
    let store = {
        let store: Box<dyn Store> = Box::new(InMemoryStore::new());
        Mutex::new(store)
    };

    let data = web::Data::new(AppData {
        store,
        tochka_client,
    });

    // TODO: for wrong entries - return error
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(create_beneficiary_ul)
        // .service(list_beneficiary)
        // .service(get_beneficiary)
        // .service(create_charity_project)
        // .service(list_all_projects)
        // .service(list_beneficiary_projects)
        // .service(get_charity_project)
        // .service(upload_document_beneficiary)
        // .service(get_document)
        // .service(create_deal)
        // .service(upload_document_deal)
        // .service(update_deal)
        // .service(list_deals)
        // .service(get_deal)
        // .service(donate)
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
    /// The key should be in the PEM format.
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
    use serde::de::DeserializeOwned;
    use tochka_cyclops::{tochka_io::*, utils, TochkaError};

    fn now() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};

        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs()
    }

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
                TochkaCyclopsClient::new(args.sign_system, args.sign_thumbprint, args.endpoint);
            let store = {
                let store: Box<dyn Store> = Box::new(InMemoryStore::new());
                Mutex::new(store)
            };

            let data = web::Data::new(AppData {
                store,
                tochka_client: maan_client,
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
                    "/upload_document_beneficiary" => {
                        (app_inner.service(upload_document_beneficiary), false)
                    }
                    "/get_document" => (app_inner.service(get_document), true),
                    "/upload_document_deal" => (app_inner.service(upload_document_deal), false),
                    "/create_deal" => (app_inner.service(create_deal), false),
                    "/update_deal" => (app_inner.service(update_deal), false),
                    "/list_deals" => (app_inner.service(list_deals), true),
                    "/get_deal" => (app_inner.service(get_deal), true),
                    "/donate" => (app_inner.service(donate), false),
                    "/test_send_qr_payment" => (app_inner.service(test_send_qr_payment), false),
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

    /// The test ensures that the beneficiary creation process works correctly, the data is
    /// stored and retrieved accurately, and duplicate creation attempts are handled properly.
    ///
    /// Details in steps:
    /// 1. Creates a new beneficiary with random INN and predefined data.
    /// 2. Verifies that the beneficiary data is stored correctly in the database.
    /// 3. Retrieves the beneficiary from the tochka server and asserts that the response data
    ///    is the same as in the database.
    /// 4. Attempts to create the same beneficiary again and expects an error response.
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
        let actual_beneficiary = test_service
            .data()
            .store
            .lock()
            .await
            .get_beneficiary(&id)
            .await
            .context("failed to get beneficiary from the db")?
            .expect("beneficiary was added within create_beneficiary_ul route");
        assert_eq!(actual_beneficiary.beneficiary, beneficiary_data.clone());

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

    /// The test is designed to verify the functionality of the charity-related endpoints.
    /// This includes creating a charity project, listing projects, getting a project for a specific beneficiary.
    ///
    /// Details in steps:
    /// 1. Create a new beneficiary
    /// 2. Create two charity projects for the beneficiary.
    /// 3. Verify that the projects are stored correctly in the database.
    /// 4. Retrieve from the server the list of projects for the beneficiary and check that the list contains the newly created projects.
    /// 5. Retrieve from the server the details of one of the projects and verify that the data matches the expected values.
    /// 6. Attempt to create a project with the same name and expect an error response.
    #[tokio::test]
    async fn test_charity_projects() -> anyhow::Result<()> {
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
        let beneficiary_id = match resp {
            MaanWebTestResponse::Error(tochka_error) => {
                panic!("Failed to create beneficiary. Got error: {tochka_error:#?}");
            }
            MaanWebTestResponse::Ok(v) => {
                let CreateBeneficiaryResponse::Beneficiary { id, .. } = v;

                id
            }
        };

        // Create a couple of charity projects
        let req_proj_1 = NewCharityProjectRequest {
            beneficiary_id: beneficiary_id.clone(),
            name: "Project 1".to_string(),
            description: "Project 1 is meant for testing".to_string(),
            cap: 10_000,
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
            cap: 10_000,
            collected: 0,
        };
        assert_eq!(
            actual_charity_project_1,
            Some(expected_charity_project_1.clone())
        );

        let req_proj_2 = NewCharityProjectRequest {
            beneficiary_id: beneficiary_id.clone(),
            name: "Project 2".to_string(),
            description: "Project 2 is meant for testing".to_string(),
            cap: 10_000,
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
            cap: 10_000,
            collected: 0,
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

    /// The test ensures that all the existing charity projects are listed correctly.
    ///
    /// Details in steps:
    /// 1. Create two new beneficiaries.
    /// 2. Create a couple of charity projects for each beneficiary.
    /// 3. Retrieve from the server the list of all charity projects and check that
    /// the list contains the newly created projects.
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
            cap: 10_000,
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
            cap: 10_000,
            collected: 0,
        };

        let req_proj_2 = NewCharityProjectRequest {
            beneficiary_id: beneficiary_id_2.clone(),
            name: "Project 2".to_string(),
            description: "Project 2 is meant for testing".to_string(),
            cap: 10_000,
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
            cap: 10_000,
            collected: 0,
        };

        let req_proj_3 = NewCharityProjectRequest {
            beneficiary_id: beneficiary_id_2.clone(),
            name: "Project 3".to_string(),
            description: "Project 3 is meant for testing".to_string(),
            cap: 10_000,
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
            cap: 10_000,
            collected: 0,
        };

        // List all charity projects
        let resp = test_service
            .send_req::<ListAllCharityProjectsResponse>((), "/list_all_projects")
            .await;
        let mut actual_projects = resp
            .ok()
            .cloned()
            .expect("internal error: failed to list all charity projects")
            .projects;

        // Check that the created projects are in the list
        let mut expected_projects = vec![
            beneficiary_1_proj_1,
            beneficiary_1_proj_2,
            beneficiary_2_proj_1,
        ];
        // Need to sort, as the order can differ
        expected_projects.sort();
        actual_projects.sort();
        assert_eq!(actual_projects, expected_projects);

        Ok(())
    }

    #[tokio::test]
    async fn test_document_beneficiary() -> anyhow::Result<()> {
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
        let beneficiary_id = match resp {
            MaanWebTestResponse::Error(tochka_error) => {
                panic!("Failed to create beneficiary. Got error: {tochka_error:#?}");
            }
            MaanWebTestResponse::Ok(v) => {
                let CreateBeneficiaryResponse::Beneficiary { id, .. } = v;

                id
            }
        };

        log::debug!("Wait until beneficiary is added to ms");
        loop {
            let beneficiary = test_service
                .data()
                .store
                .lock()
                .await
                .get_beneficiary(&beneficiary_id)
                .await?
                .expect("beneficiary was added to the store after create_beneficiary_ul req");

            if beneficiary.is_addded_to_ms {
                log::debug!("Beneficiary is added to ms");
                break;
            }
        }

        // Upload document for beneficiary
        let pdf_content = r#"%PDF-1.4
            1 0 obj
            << /Type /Catalog /Pages 2 0 R >>
            endobj
            2 0 obj
            << /Type /Pages /Kids [3 0 R] /Count 1 >>
            endobj
            3 0 obj
            << /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R >>
            endobj
            4 0 obj
            << /Length 44 >>
            stream
            BT
            /F1 24 Tf
            100 700 Td
            (Test PDF Document) Tj
            ET
            endstream
            endobj
            5 0 obj
            << /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>
            endobj
            xref
            0 6
            0000000000 65535 f 
            0000000010 00000 n 
            0000000053 00000 n 
            0000000100 00000 n 
            0000000173 00000 n 
            0000000321 00000 n 
            trailer
            << /Size 6 /Root 1 0 R >>
            startxref
            377
            %%EOF
            "#;
        let req = UploadDocumentBeneficiaryRequest {
            b64_document: base64::encode(pdf_content),
            beneficiary_id: beneficiary_id.clone(),
            document_number: "12345".to_string(),
            document_date: "2023-10-01".to_string(),
            content_type: "application/pdf".to_string(),
        };
        let resp = test_service
            .send_req::<UploadDocumentBeneficiaryResponse>(&req, "/upload_document_beneficiary")
            .await;

        // Check response
        let document_id = match resp {
            MaanWebTestResponse::Error(tochka_error) => {
                panic!("Failed to upload document. Got error: {tochka_error:#?}");
            }
            MaanWebTestResponse::Ok(resp) => resp.document_id,
        };

        // Verify document is stored correctly
        let stored_document = test_service
            .data()
            .store
            .lock()
            .await
            .get_beneficiary_document(&beneficiary_id, &document_id)
            .await
            .expect("failed to get document from db");
        assert!(stored_document.is_some());

        let req = GetDocumentRequest { document_id };
        let resp = test_service
            .send_req::<GetDocumentResponse>(&req, "/get_document")
            .await;
        assert!(resp.is_ok());

        let succes_added = resp
            .ok()
            .map(|d| d.success_added)
            .expect("failed to get document");
        assert!(succes_added);

        Ok(())
    }

    #[tokio::test]
    async fn test_full() -> anyhow::Result<()> {
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
        let beneficiary_id = match resp {
            MaanWebTestResponse::Error(tochka_error) => {
                panic!("Failed to create beneficiary. Got error: {tochka_error:#?}");
            }
            MaanWebTestResponse::Ok(v) => {
                let CreateBeneficiaryResponse::Beneficiary { id, .. } = v;
                id
            }
        };

        // Create a couple of charity projects
        let req_proj_1 = NewCharityProjectRequest {
            beneficiary_id: beneficiary_id.clone(),
            name: "Project 1".to_string(),
            description: "Project 1 is meant for testing".to_string(),
            cap: 10_000,
        };
        let resp = test_service
            .send_req::<NewCharityProjectResponse>(req_proj_1.clone(), "/create_charity_project")
            .await;
        let project_1_id = resp
            .ok()
            .expect("internal error: failed to create charity project 1")
            .id
            .clone();

        // Donate to the project
        let donate_req = DonateRequest {
            project: project_1_id.clone(),
            donator: "Donator".to_string(),
            amount: 3000,
        };
        let resp = test_service
            .send_req::<DonateResponse>(&donate_req, "/donate")
            .await;

        // Check response
        let qr_id = match resp {
            MaanWebTestResponse::Error(tochka_error) => {
                panic!("Failed to donate. Got error: {tochka_error:#?}");
            }
            MaanWebTestResponse::Ok(v) => v.qr_code,
        };

        // Proceed qr-code payment for a test
        let req = TestSendQrPaymentRequest {
            qrc_id: qr_id.clone(),
            amount: 3000,
            qrc_type: "02".to_string(),
            creditor_bank_id: "100000000284".to_string(),
        };
        let resp = test_service
            .send_req::<TestSendQrPaymentResponse>(&req, "/test_send_qr_payment")
            .await;

        // Check response
        if let MaanWebTestResponse::Error(tochka_error) = resp {
            panic!("Failed to proceed qr-code payment. Got error: {tochka_error:#?}");
        }

        log::debug!("Wait until funds are identified");
        loop {
            let project = test_service
                .data()
                .store
                .lock()
                .await
                .get_charity_project(&project_1_id)
                .await?
                .expect("beneficiary was added to the store after create_beneficiary_ul req");

            if project.collected == 3000 {
                log::debug!("Funds are identified");
                break;
            };
        }

        // Create a deal
        let create_deal_req = CreateDealRequest {
            amount: 1000.0,
            ext_key: utils::new_uuid_v4().to_string(),
            payers: vec![PaymentOwner {
                virtual_account: project_1_id,
                amount: 0,
            }],
            recipients: vec![DealRecipient::PaymentContract {
                number: 1,
                amount: 1000.0,
                purpose_nds: None,
                account: "40702810238030000904".to_string(),
                bank_code: "046577964".to_string(),
                name: "ООО \"Петруня\"".to_string(),
                inn: "6671217676".to_string(),
                kpp: Some("667101001".to_string()),
                document_number: Some("42".to_string()),
                purpose: Some(format!("test_document_{}, без НДС", now())),
                code_purpose: None,
                identifier: beneficiary_id.clone(),
            }],
        };
        let resp = test_service
            .send_req::<CreateDealResponse>(&create_deal_req, "/create_deal")
            .await;

        // Check response
        let deal_id = match resp {
            MaanWebTestResponse::Error(tochka_error) => {
                panic!("Failed to create deal. Got error: {tochka_error:#?}");
            }
            MaanWebTestResponse::Ok(v) => v.deal_id,
        };

        // Upload document for deal
        let pdf_content = r#"%PDF-1.4
            1 0 obj
            << /Type /Catalog /Pages 2 0 R >>
            endobj
            2 0 obj
            << /Type /Pages /Kids [3 0 R] /Count 1 >>
            endobj
            3 0 obj
            << /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R >>
            endobj
            4 0 obj
            << /Length 44 >>
            stream
            BT
            /F1 24 Tf
            100 700 Td
            (Test PDF Document) Tj
            ET
            endstream
            endobj
            5 0 obj
            << /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>
            endobj
            xref
            0 6
            0000000000 65535 f 
            0000000010 00000 n 
            0000000053 00000 n 
            0000000100 00000 n 
            0000000173 00000 n 
            0000000321 00000 n 
            trailer
            << /Size 6 /Root 1 0 R >>
            startxref
            377
            %%EOF
            "#;
        let req = UploadDocumentDealRequest {
            b64_document: base64::encode(pdf_content),
            beneficiary_id: beneficiary_id.clone(),
            deal_id: deal_id.clone(),
            document_number: "12345".to_string(),
            document_date: "2023-10-01".to_string(),
            content_type: "application/pdf".to_string(),
        };
        let resp = test_service
            .send_req::<UploadDocumentDealResponse>(&req, "/upload_document_deal")
            .await;

        // Check response
        let document_id = match resp {
            MaanWebTestResponse::Error(tochka_error) => {
                panic!("Failed to upload document. Got error: {tochka_error:#?}");
            }
            MaanWebTestResponse::Ok(resp) => resp.document_id,
        };

        // Get the document
        let req = GetDocumentRequest { document_id };
        let resp = test_service
            .send_req::<GetDocumentResponse>(&req, "/get_document")
            .await;
        assert!(resp.is_ok());

        let success_added = resp
            .ok()
            .map(|d| d.success_added)
            .expect("failed to get document");
        assert!(success_added);

        // Update the deal (update amount)
        let update_deal_req = UpdateDealRequest {
            deal_id: deal_id.clone(),
            deal_data: UpdateDealData {
                amount: 2000.0,
                payers: create_deal_req.payers.clone(),
                recipients: create_deal_req.recipients.clone(),
            },
        };
        let resp = test_service
            .send_req::<UpdateDealResponse>(&update_deal_req, "/update_deal")
            .await;
        assert!(resp.is_ok());

        // Get the updated deal
        let req = GetDealRequest {
            deal_id: deal_id.clone(),
        };
        let resp = test_service
            .send_req::<GetDealResponse>(&req, "/get_deal")
            .await;
        assert!(resp.is_ok());

        let deal = resp.ok().expect("failed to get deal");
        assert_eq!(deal.deal.amount, 2000.0);

        // List all deals
        let list_deals_req = ListDealsRequest {
            per_page: None,
            page: None,
            field_names: vec!["amount".to_string(), "ext_key".to_string()],
            filters: Default::default(),
        };
        let resp = test_service
            .send_req::<ListDealsResponse>(&list_deals_req, "/list_deals")
            .await;
        assert!(resp.is_ok());

        let deals = &resp.ok().expect("failed to list deals").deals;
        let found_deal = &deals
            .iter()
            .find(|deal| deal.id == deal_id)
            .expect("deal not found");
        assert_eq!(found_deal.amount.expect("requested field"), 2000.0);
        assert_eq!(
            found_deal.ext_key.clone().expect("requested field"),
            create_deal_req.ext_key
        );

        // Execute deal
        let execute_deal_req = ExecuteDealRequest {
            deal_id: deal_id.clone(),
        };
        let resp = test_service
            .send_req::<ExecuteDealResponse>(&execute_deal_req, "/execute_deal")
            .await;
        assert!(resp.is_ok());

        loop {
            // Wait until deal gets closed status

            let req = GetDealRequest {
                deal_id: deal_id.clone(),
            };
            let resp = test_service
                .send_req::<GetDealResponse>(&req, "/get_deal")
                .await;

            let deal = match resp {
                MaanWebTestResponse::Error(tochka_error) => {
                    panic!("Failed to get deal. Got error: {tochka_error:#?}");
                }
                MaanWebTestResponse::Ok(v) => v.deal,
            };

            let status = deal.status.to_lowercase();
            if status.contains("closed") {
                break;
            }

            if status.contains("rejected")
                || status.contains("correction")
                || status.contains("cancel")
            {
                panic!("Deal status reached unexpected state - {status}. Deal: {deal:#?}");
            }
        }

        Ok(())
    }
}
