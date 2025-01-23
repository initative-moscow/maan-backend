use anyhow::Result;
use rsa::{
    pkcs8::DecodePrivateKey,
    sha2::{Digest, Sha256},
    Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey,
};

#[derive(Debug, Clone)]
pub struct Signer {
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
}

impl Signer {
    pub fn new(pem_string: String) -> Result<Self> {
        let private_key = RsaPrivateKey::from_pkcs8_pem(&pem_string)?;
        let public_key = private_key.to_public_key();

        Ok(Signer {
            private_key,
            public_key,
        })
    }

    pub fn sign_raw_data(&self, data: impl AsRef<[u8]>) -> Result<Vec<u8>> {
        let digest = Sha256::digest(data.as_ref());

        self.private_key
            .sign(Pkcs1v15Sign::new::<Sha256>(), &digest)
            .map_err(Into::into)
    }

    pub fn verify_raw_data(
        &self,
        signature: impl AsRef<[u8]>,
        data: impl AsRef<[u8]>,
    ) -> Result<()> {
        let digest = Sha256::digest(data.as_ref());

        self.public_key
            .verify(Pkcs1v15Sign::new::<Sha256>(), &digest, signature.as_ref())
            .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use rsa::pkcs8::{DecodePublicKey, EncodePublicKey};

    use super::*;

    // TODO change tests

    // #[test]
    // fn integrational_test_rsa_pub_key_from_private() {
    //     let private_key =
    //         RsaPrivateKey::from_pkcs8_pem(PRIVATE_KEY).expect("failed private key creation");
    //     let public_key_expected =
    //         RsaPublicKey::from_public_key_pem(PUBLIC_KEY).expect("failed pub key creation");
    //     let public_key_actual = private_key.to_public_key();

    //     assert_eq!(public_key_expected, public_key_actual);

    //     let public_key_actual_pem = public_key_actual
    //         .to_public_key_pem(Default::default())
    //         .expect("failed public key pem creation");
    //     assert_eq!(PUBLIC_KEY, public_key_actual_pem);
    // }

    // #[test]
    // fn integrational_test_sign_and_verify() {
    //     let signer = Signer::new().expect("failed signer creation");
    //     let signing_data = b"hello_world";

    //     let signature = signer
    //         .sign_raw_data(signing_data)
    //         .expect("failed signing data");
    //     assert!(signer.verify_raw_data(signature, signing_data).is_ok());
    // }
}
