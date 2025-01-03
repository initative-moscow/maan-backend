// TODO:
// 1. use anyhow for the error

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use clap::Parser;
use maan_core::tochka::{
    create_beneficiary::{BeneficiaryData, CreateBeneficiaryResponse, CreateBeneficiaryUlRequest},
    TochkaApiRequest, TochkaApiResponse,
};
use maan_core::{MaanClient, Signer};
use std::{collections::BTreeMap, fs, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
struct AppData {
    store: Arc<Mutex<BTreeMap<String, BeneficiaryData>>>,
    maan_client: MaanClient,
    signer: Signer,
}

#[post("/create_beneficiary")]
async fn create_beneficiary(
    data: web::Data<AppData>,
    create_beneficiary_req: web::Json<CreateBeneficiaryUlRequest>,
) -> std::io::Result<impl Responder> {
    println!("{create_beneficiary_req:#?}");

    Ok(web::Json(create_beneficiary_req.0))
}

#[actix_web::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let private_key_string = fs::read_to_string(args.private_key_path)?;
    let signer = Signer::new(private_key_string)?;
    let maan_client = MaanClient::new(args.sign_system, args.sign_thumbprint, args.endpoint);
    let store = Arc::new(Mutex::const_new(BTreeMap::new()));
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
    #[arg(short = 'k', long = "private_key")]
    private_key_path: PathBuf,

    /// Sign system for the Tochka.
    #[arg(short = 's')]
    sign_system: String,

    /// Sign thumbprint for the Tochka.
    #[arg(short = 't')]
    sign_thumbprint: String,

    /// Tochka service API endpoint.
    endpoint: String,
}
