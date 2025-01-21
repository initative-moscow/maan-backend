// TODO:
// 1. use anyhow for the error

mod db;
mod error;

use actix_web::{post, get, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use db::{InMemoryStore, Store};
use error::AnyhowResponseError;
use maan_core::tochka::{
    create_beneficiary::{BeneficiaryData, CreateBeneficiaryResponse, CreateBeneficiaryUlRequest}, list_beneficiary::{ListBeneficiaryRequest, ListBeneficiaryResponse}, TochkaApiRequest, TochkaApiResponse, TochkaApiResponsePayload
};
use maan_core::{MaanClient, Signer};
use std::{collections::BTreeMap, fs, path::PathBuf, sync::Arc, fmt::Debug};
use tokio::sync::Mutex;

struct AppData {
    store: Box<dyn Store>,
    maan_client: MaanClient,
    signer: Signer,
}

#[post("/create_beneficiary")]
async fn create_beneficiary(
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
            data_clone.maan_client
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
            let CreateBeneficiaryResponse::Beneficiary { id, ..  } = result;
            data.store.store_beneficiary(id, beneficiary_data).await?;

            Ok(HttpResponse::Ok().finish())
        }
        TochkaApiResponsePayload::Error { error } => Ok(HttpResponse::InternalServerError().json(error)),
    }
}

#[get("/list_beneficiary")]
async fn list_beneficiary(data: web::Data<AppData>, list_beneficiary_req: web::Json<ListBeneficiaryRequest>) -> Result<impl Responder, AnyhowResponseError> {
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
        TochkaApiResponsePayload::Result { result } => {
            Ok(HttpResponse::Ok().json(result))
        }
        TochkaApiResponsePayload::Error { error } => Ok(HttpResponse::InternalServerError().json(error)),
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
    let store = Box::new(InMemoryStore::new());
    let data = web::Data::new(AppData {
        store,
        maan_client,
        signer,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(create_beneficiary)
            .service(list_beneficiary)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .map_err(Into::into)
}

#[derive(Debug, Parser)]
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
