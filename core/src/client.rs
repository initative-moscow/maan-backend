use crate::{utils, Signer};
use anyhow::Result;
use reqwest::{
    blocking::{Client, Response},
    Method,
};

const PLATFORM_ID: &str = "maan";
const CERT_THUMBPRINT: &str = "ffffffffffffffffffffffffffffffffffffffff";

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
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert!(maan_client.send_request(&signer, request_bytes).is_ok())
    }
}
