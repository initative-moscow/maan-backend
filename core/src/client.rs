use crate::{utils, Signer};
use anyhow::Result;
use reqwest::{
    blocking::{Client, Response},
    Method,
};
use std::collections::HashMap;

// Known endpoint for cyclops: "https://pre.tochka.com/api/v1/cyclops/v2/jsonrpc";

#[derive(Debug, Clone)]
pub struct MaanClient {
    sign_system: String,
    sign_thumbprint: String,
    endpoint: String,
}

impl MaanClient {
    pub fn new(sign_system: String, sign_thumbprint: String, endpoint: String) -> Self {
        MaanClient {
            sign_system,
            sign_thumbprint,
            endpoint,
        }
    }

    pub fn send_request(
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
            .request(Method::POST, &self.endpoint)
            .header("sign-system", &self.sign_system)
            .header("sign-thumbprint", &self.sign_thumbprint)
            .header("sign-data", &b64_body_signature)
            .header("Content-Type", "application/json")
            .body(body);

        log::debug!("Sending request: {:#?}", request);

        request.send().map_err(Into::into)
    }

    pub fn send_request_tenders(
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
            .request(
                Method::POST,
                "https://pre.tochka.com/api/v1/tender-helpers/jsonrpc",
            )
            .header("sign-system", &self.sign_system)
            .header("sign-thumbprint", &self.sign_thumbprint)
            .header("sign-data", &b64_body_signature)
            .header("Content-Type", "application/json")
            .body(body);

        log::debug!("Sending request: {:#?}", request);

        request.send().map_err(Into::into)
    }

    pub fn upload_document_beneficiary(
        &self,
        signer: &Signer,
        b64_document: String,
        beneficiary_id: String,
        document_number: String,
        document_date: String,
        content_type: String,
    ) -> Result<Response> {
        let document_bytes = utils::base64_decode(b64_document)?;
        let mut query_params = HashMap::new();
        query_params.insert("beneficiary_id", beneficiary_id);
        query_params.insert("document_type", "contract_offer".to_string());
        query_params.insert("document_number", document_number);
        query_params.insert("document_date", document_date);

        let b64_document_signature = {
            let signed_body = signer.sign_raw_data(&document_bytes).unwrap();
            utils::base64_encode(signed_body)
        };

        let client = Client::new();
        let request = client
            .request(
                Method::POST,
                "https://pre.tochka.com/api/v1/cyclops/upload_document/beneficiary",
            )
            .query(&query_params)
            .header("sign-system", &self.sign_system)
            .header("sign-thumbprint", &self.sign_thumbprint)
            .header("sign-data", &b64_document_signature)
            .header("Content-Type", content_type)
            .body(document_bytes);

        log::debug!("Sending request: {:#?}", request);

        request.send().map_err(Into::into)
    }

    pub fn upload_document_deal(
        &self,
        signer: &Signer,
        b64_document: String,
        beneficiary_id: String,
        deal_id: String,
        document_number: String,
        document_date: String,
        content_type: String,
    ) -> Result<Response> {
        let document_bytes = utils::base64_decode(b64_document)?;
        let mut query_params = HashMap::new();
        query_params.insert("beneficiary_id", beneficiary_id);
        query_params.insert("deal_id", deal_id);
        query_params.insert("document_type", "service_agreement".to_string());
        query_params.insert("document_date", document_date);
        query_params.insert("document_number", document_number);

        let b64_document_signature = {
            let signed_body = signer.sign_raw_data(&document_bytes).unwrap();
            utils::base64_encode(signed_body)
        };

        let client = Client::new();
        let request = client
            .request(
                Method::POST,
                "https://pre.tochka.com/api/v1/cyclops/upload_document/deal",
            )
            .query(&query_params)
            .header("sign-system", &self.sign_system)
            .header("sign-thumbprint", &self.sign_thumbprint)
            .header("sign-data", &b64_document_signature)
            .header("Content-Type", content_type)
            .body(document_bytes);

        log::debug!("Sending request: {:#?}", request);

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
}
