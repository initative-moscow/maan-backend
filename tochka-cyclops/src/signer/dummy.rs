//! Dummy signer.

use crate::client::RequestSigner;

pub struct DummySigner;

impl<Body> RequestSigner<Body> for DummySigner {
    type Error = ();

    fn sign_request_body(&self, _: Body) -> Result<Vec<u8>, Self::Error> {
        Ok(vec![])
    }
}
