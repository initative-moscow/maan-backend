use anyhow::Error as AnyhowError;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

trait RSA2048SHA256SigningService {
    type Output;
    type Error;

    fn sign_data<T: Into<Vec<u8>>>(&self, data: T) -> Result<Self::Output, Self::Error>;
}

struct YandexKMS {
    key_id: String,
    iam_token: String,
}

impl YandexKMS {
    fn new<T: Into<String>>(key_id: T, iam_token: T) -> Self {
        let (key_id, iam_token) = (key_id.into(), iam_token.into());

        Self { key_id, iam_token }
    }

    fn sign_data_endpoint(&self) -> String {
        format!(
            "https://kms.yandex/kms/v1/asymmetricSignatureKeys/{key_id}:sign",
            key_id = self.key_id
        )
    }
}

impl RSA2048SHA256SigningService for YandexKMS {
    // todo provide own type
    type Error = AnyhowError;
    // todo actually returned data can be other, if error happens on the service side
    type Output = SignDataReponse;

    fn sign_data<T: Into<Vec<u8>>>(&self, data: T) -> Result<Self::Output, Self::Error> {
        let data = data.into();
        let client = Client::new();
        let response = client
            .post(self.sign_data_endpoint())
            .bearer_auth(&self.iam_token)
            .body(data)
            .send()
            .map_err(AnyhowError::from)?;

        serde_json::from_reader(response).map_err(AnyhowError::from)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SignDataReponse {
    #[serde(rename = "keyId")]
    key_id: String,
    signature: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[derive(Serialize)]
    struct SignDataRequest {
        message: String,
    }

    #[test]
    // todo test signature using openssl against pub-key and data
    fn test_kms_sign_data() {
        let (key_id, iam_token) = {
            dotenv::dotenv().expect("internal error: env settting failed");

            // todo must provide with a cli config.
            // actually find a way to obtain these data from yc.
            let key_id = env::var("KEY_ID").expect("KEY_ID var isn't set");
            let iam_token = env::var("IAM_TOKEN").expect("IAM_TOKEN var isn't set");

            (key_id, iam_token)
        };

        let data = {
            // hello base64 encoded
            let deserialized = SignDataRequest {
                message: "dGVzdA==".into(),
            };
            serde_json::to_vec(&deserialized).expect("Serialize implemented; qed.")
        };

        let kms = YandexKMS::new(key_id.clone(), iam_token);
        let resp = kms.sign_data(data).expect("failed signing data");

        assert_eq!(resp.key_id, key_id);
    }
}
