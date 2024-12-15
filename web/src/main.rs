// TODO:
// 1. use anyhow for the error

use actix_web::{App, HttpServer, HttpResponse, Responder, post};
use maan_core::tochka::{
    create_beneficiary::{CreateBeneficiaryUlRequest, CreateBeneficiaryResponse, BeneficiaryData},
    TochkaApiRequest, TochkaApiResponse,
};

#[post("/create_beneficiary")]
async fn create_beneficiary(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create_beneficiary)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}