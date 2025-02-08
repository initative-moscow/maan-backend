//! A default signer implementation for Tochka Cyclops API Client.
//!
//! The signer is used to sign the request body before sending it to the Tochka API.
//! The default signer provided by the crate uses RSA PKCS1v15 signing algorithm with SHA256 hash function.

use crate::client::RequestSigner;
use rsa::{
    pkcs8::DecodePrivateKey,
    sha2::{Digest, Sha256},
    Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey,
};
use std::{fs, path::PathBuf};

/// Signer with RSA PKCS1v15 signing algorithm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsaSigner {
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
}

impl RsaSigner {
    /// Create a new signer from a PEM file.
    pub fn new(pem_path: PathBuf) -> Result<Self, SignerError> {
        let pem_string = fs::read_to_string(pem_path)?;

        Self::from_pem(pem_string)
    }

    /// Create a new signer from a private key pem string.
    pub fn from_pem(pem: String) -> Result<Self, SignerError> {
        let private_key = RsaPrivateKey::from_pkcs8_pem(&pem).map_err(rsa::Error::from)?;
        let public_key = private_key.to_public_key();

        Ok(RsaSigner {
            private_key,
            public_key,
        })
    }

    /// Sign raw data with the private key.
    pub fn sign_raw_data(&self, data: impl AsRef<[u8]>) -> Result<Vec<u8>, SignerError> {
        let digest = Sha256::digest(data.as_ref());
        let res = self
            .private_key
            .sign(Pkcs1v15Sign::new::<Sha256>(), &digest)?;

        Ok(res)
    }

    /// Verify raw data signature with the public key.
    pub fn verify_raw_data(
        &self,
        signature: impl AsRef<[u8]>,
        data: impl AsRef<[u8]>,
    ) -> Result<(), SignerError> {
        let digest = Sha256::digest(data.as_ref());
        self.public_key
            .verify(Pkcs1v15Sign::new::<Sha256>(), &digest, signature.as_ref())?;

        Ok(())
    }
}

impl<T: AsRef<[u8]>> RequestSigner<T> for RsaSigner {
    type Error = SignerError;

    fn sign_request_body(&self, body: T) -> Result<Vec<u8>, Self::Error> {
        self.sign_raw_data(body)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SignerError {
    #[error("Failed to read private key from file: {0}")]
    IO(#[from] std::io::Error),
    #[error("Rsa error: {0}")]
    Rsa(#[from] rsa::Error),
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use rsa::pkcs8::{DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding};
    use std::{fs, io::Write};

    use super::*;

    const TEST_PRIVATE_KEY: &str = "-----BEGIN PRIVATE KEY-----
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQDj7F25PAkT3b4U
xK/r7NeiTPG8lRfDPq9Sp9Pr42UbE0j2Q6eRoG1XEwDpjTUpN+kurNFSH7oqCQSc
awRwfu927HxszvWCC/eEJcg0cJSJ68ZhNa93luuwRxTiBSnowyvFNsc1ilP0fxGf
XYdTdxjPwsd8KJO1ERD4R1uIpiZDG+2CwCBp9KHzJmUyBxc2Vdkrst20riHvGYa6
jFJHAXoKxAXxhXD7IZtsMhh4SmitbAtqaq0ypwifioGhJ1P9CpYxFISQBR2bNdEs
49VCioT5NDIHV5jv24bHSd93EFPQSrA9abNSht98v9BuzH0o5w3JgkLUoZgUxSRL
0agb1V+xAgMBAAECggEBAM1cBeJFVoSA0ahSw5NV9cA1hcV9rEht7XgS4R3z6AAq
Mx44PP44RuwiojrM0S5PQxmb+on9LjZ7o5zvy7M0m7xSPZWoL4q6D40Qob+mBB/0
wOyLe4rL/5X3pbg0INupJoHt5jW2agisiQtHMQwiDcf8XtLemJ+XPewGF1IbKfRh
vOrbAT7XyCJf9UUvCE0/fEQXimcpRz/TaMqrIPbQOWr2TiLiNanKUHI8PtUr2ZXk
H9Lpb4JHmC9aiWY9IbJRsCSHEts0Vx9areKyX0vUbDC7EFXcyCQ7YBRBZglWF2dJ
jvZx8lkoT41m/sTJQfurpgrVpcB9t6xGqCt+FXj0E3ECgYEA6bNW7rWqeJ8rL4oL
R3DL7gMGn+OrElGJaedsVDvZkhgxOKJdPRp94v41wj9AAD/ynO9FZGvrNRvLwwwn
NhR5A2n49iEz389H/r6zGgZU/f+YPbEyICOTOMuz5hjrehFty3+JPATsNIxzoEpm
hw19PaznERNaS3kZO4YYluUvhaUCgYEA+avni0qQBrazroEdmXcltDkFuvRIG93n
5Irc+AxJXq8ntX1jV/PEcEPKbpKmadMiz9YTJ/5N3shSySyh6vzJhhdsi0Mz2kMK
R0WNvz6aexql5kb1gm97OOsyLlWShQAMwmfMVnZQnKr2u4sDKLOI6o0X6X8fdNTw
xCaWV0pZjB0CgYAMBGbR+5F6hmTIGwcdc1VpNcqfaiuf04WHZpkRc8pqUsglK2Q0
Aiq7A2tsQ6hc2uz02PDuiwYbQwSlUPirNT5LyKU+stJiDdyb4t5+1hiEvyHq+jOj
p3ComQD1Mg7Zxg+pSTObXH1w7k/7zBedljn8c+ml3SLlgqKjhu+4wqxA6QKBgF9I
VqZW/15AjZQ0XEp8KRx9go1Vuss/xcb3o9raPYnwCJR/1ND1C+vYQ0Itn2rVk/yD
c24Y5Dj4dHeoG+clL/eHqvn+3KQYX6zRg4YP6z697cBTJlDwILOZNt5t8+vkF/p9
SINaxer0aBMsuzjmQ4NX818+D5Azz+rb2xZXHpOVAoGBAMpr7PHC8pYQzVXFW0Qw
iPqFqLa9fDrfE/XKu0nI+yOxWpTWhNnuiZKW3g04zihLuJKoVoJM22E+z3VBWBbL
5sM+yddo4uyZp5InButauAjS5lUyyRG9+lOEe/dbMQNe1ZYFnIin9AZmaQJyvpU/
PmAXAQ1AlhbZxYNrDyIKxjpr
-----END PRIVATE KEY-----
";

    const TEST_PUBLIC_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA4+xduTwJE92+FMSv6+zX
okzxvJUXwz6vUqfT6+NlGxNI9kOnkaBtVxMA6Y01KTfpLqzRUh+6KgkEnGsEcH7v
dux8bM71ggv3hCXINHCUievGYTWvd5brsEcU4gUp6MMrxTbHNYpT9H8Rn12HU3cY
z8LHfCiTtREQ+EdbiKYmQxvtgsAgafSh8yZlMgcXNlXZK7LdtK4h7xmGuoxSRwF6
CsQF8YVw+yGbbDIYeEporWwLamqtMqcIn4qBoSdT/QqWMRSEkAUdmzXRLOPVQoqE
+TQyB1eY79uGx0nfdxBT0EqwPWmzUobffL/Qbsx9KOcNyYJC1KGYFMUkS9GoG9Vf
sQIDAQAB
-----END PUBLIC KEY-----
";

    #[test]
    fn integrational_test_rsa_private_key() {
        // Generate a new private key
        let mut rng = rand::rngs::OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("failed private key creation");

        // Encode the private key to PEM format
        let pem = private_key
            .to_pkcs8_pem(LineEnding::default())
            .expect("failed to encode key to PEM");
        println!("{:?}", pem);

        // Write the PEM to a temp file
        let mut tmp_file = tempfile::NamedTempFile::new().expect("failed to create temp file");
        tmp_file
            .write_all(pem.as_bytes())
            .expect("failed to write to temp file");

        // Read the file contents into a String
        let pem_string =
            fs::read_to_string(tmp_file.path()).expect("failed to read file to string");

        // Instantiate RsaPrivateKey from the PEM string
        let private_key_from_pem = RsaPrivateKey::from_pkcs8_pem(&pem_string)
            .expect("failed to create RsaPrivateKey from PEM string");

        // Ensure the keys are the same
        assert_eq!(private_key, private_key_from_pem);
        // assert_eq!(private_key, private_key_from_bytes);
    }

    #[test]
    fn integrational_test_rsa_pub_key_from_private() {
        let private_key =
            RsaPrivateKey::from_pkcs8_pem(TEST_PRIVATE_KEY).expect("failed private key creation");
        let public_key_expected =
            RsaPublicKey::from_public_key_pem(TEST_PUBLIC_KEY).expect("failed pub key creation");
        let public_key_actual = private_key.to_public_key();

        assert_eq!(public_key_expected, public_key_actual);

        let public_key_actual_pem = public_key_actual
            .to_public_key_pem(Default::default())
            .expect("failed public key pem creation");
        assert_eq!(TEST_PUBLIC_KEY, public_key_actual_pem);
    }

    #[test]
    fn test_sign_and_verify() -> Result<()> {
        let signer = RsaSigner::from_pem(TEST_PRIVATE_KEY.to_string())?;
        let signing_data = b"hello_world";

        let signature = signer
            .sign_raw_data(signing_data)
            .expect("failed signing data");
        assert!(signer.verify_raw_data(signature, signing_data).is_ok());

        Ok(())
    }
}
