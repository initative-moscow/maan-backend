use crate::signer::RsaSigner;
use crate::{utils, TochkaApiRequest, TochkaApiResponse};
use anyhow::Result as AnyhowResult;
use reqwest::{Client, Method, Response};
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, error::Error as StdError};

/// Signer which signs the request body for Tochka Cyclops API.
pub trait RequestSigner<Body> {
    type Error;

    fn sign_request_body(&self, body: Body) -> Result<Vec<u8>, Self::Error>;
}

/// Tochka Cyclops API client.
///
/// The client under the hood uses [`reqwest`] to send http requests.
#[derive(Debug, Clone)]
pub struct TochkaCyclopsClient<S> {
    sign_system: String,
    sign_thumbprint: String,
    endpoint: String,
    signer: S,
}

impl<S> TochkaCyclopsClient<S> {
    /// Instantiate a new client with a signer.
    ///
    /// The sign system and sign thumbprint are provided by Tochka during onboarding process.
    pub fn new(sign_system: String, sign_thumbprint: String, endpoint: String, signer: S) -> Self {
        TochkaCyclopsClient {
            sign_system,
            sign_thumbprint,
            endpoint,
            signer,
        }
    }

    /// Check if the client is using the testing endpoint.
    ///
    /// Testing endpoint contains "pre.tochka.com".
    pub fn is_testing_client(&self) -> bool {
        self.endpoint.contains("pre.tochka.com")
    }

    pub async fn send_request<T: Serialize, R: DeserializeOwned>(
        &self,
        method: &'static str,
        params: T,
    ) -> AnyhowResult<TochkaApiResponse<R>>
    where
        S: RequestSigner<Vec<u8>>,
        <S as RequestSigner<Vec<u8>>>::Error: StdError + Send + Sync + 'static,
    {
        let params = serde_json::to_value(params).unwrap_or_else(|e| {
            // TODO create Sealed trait for params, so only tochka IO can be used
            unreachable!("internal error: failed converting known params to serde_json::Value: {e}")
        });
        let tochka_req = TochkaApiRequest {
            params,
            jsonrpc: "2.0".to_string(),
            id: utils::new_uuid_v4().to_string(),
            method: method.to_string(),
        };

        log::debug!("Sending Tochka Cyclops request: {tochka_req:?}");

        let req_bytes = serde_json::to_vec(&tochka_req).unwrap_or_else(|e| {
            unreachable!("internal error: failed serializing known params to bytes: {e}")
        });
        let resp = self.send_request_raw(req_bytes).await?;

        resp.json().await.map_err(Into::into)
    }

    /// Sends a request with a signer client's signer.
    pub async fn send_request_raw<Body>(&self, body: Body) -> AnyhowResult<Response>
    where
        Body: AsRef<[u8]> + Clone,
        S: RequestSigner<Body>,
        <S as RequestSigner<Body>>::Error: StdError + Send + Sync + 'static,
    {
        self.send_request_raw_with_signer(&self.signer, body).await
    }

    /// Low level sending request method which uses a provided signer.
    pub async fn send_request_raw_with_signer<Body>(
        &self,
        signer: &S,
        body: Body,
    ) -> AnyhowResult<Response>
    where
        Body: AsRef<[u8]> + Clone,
        S: RequestSigner<Body>,
        <S as RequestSigner<Body>>::Error: StdError + Send + Sync + 'static,
    {
        let req_body = body.as_ref().to_vec();
        let b64_body_signature = {
            let signed_body = signer.sign_request_body(body)?;
            utils::base64_encode(signed_body)
        };

        let client = Client::new();
        let request = client
            .request(Method::POST, &self.endpoint)
            .header("sign-system", &self.sign_system)
            .header("sign-thumbprint", &self.sign_thumbprint)
            .header("sign-data", &b64_body_signature)
            .header("Content-Type", "application/json")
            .body(req_body);

        log::debug!("Sending HTTP request: {:?}", request);

        request.send().await.map_err(Into::into)
    }

    pub async fn send_request_tenders(
        &self,
        signer: &RsaSigner,
        body: impl AsRef<[u8]> + Clone,
    ) -> AnyhowResult<Response> {
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

        request.send().await.map_err(Into::into)
    }

    pub async fn upload_document_beneficiary(
        &self,
        signer: &RsaSigner,
        b64_document: String,
        beneficiary_id: String,
        document_number: String,
        document_date: String,
        content_type: String,
    ) -> AnyhowResult<Response> {
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

        request.send().await.map_err(Into::into)
    }

    pub async fn upload_document_deal(
        &self,
        signer: &RsaSigner,
        b64_document: String,
        beneficiary_id: String,
        deal_id: String,
        document_number: String,
        document_date: String,
        content_type: String,
    ) -> AnyhowResult<Response> {
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

        request.send().await.map_err(Into::into)
    }
}
