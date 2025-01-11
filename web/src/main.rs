// TODO:
// 1. use anyhow for the error

mod db;
mod error;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use db::{InMemoryStore, Store};
use error::AnyhowResponseError;
use maan_core::tochka::{
    create_beneficiary::{BeneficiaryData, CreateBeneficiaryResponse, CreateBeneficiaryUlRequest},
    TochkaApiRequest, TochkaApiResponse,
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
    let f = async move {
        let v = serde_json::to_value(&create_beneficiary_req.0)?;
        println!("{v:#?}");
        let req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": maan_core::utils::new_uuid_v4().to_string(),
            "method": "create_beneficiary_ul",
            "params": v,
        });
        println!("Request {req:#?}");
        let bytes = serde_json::to_vec(&req).expect("failed to serialize request");
        let resp = web::block(move || {
            data.maan_client
                .send_request(&data.signer, bytes)
                .unwrap()
                .json::<TochkaApiResponse<CreateBeneficiaryResponse>>()
        }).await;
        println!("{resp:#?}");

        Ok::<(), anyhow::Error>(())
    };

    f.await?;
   
    Ok("hello world!")
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
