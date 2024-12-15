use base64::{prelude::BASE64_STANDARD, DecodeError, Engine};
use uuid::Uuid;

pub(crate) fn new_uuid_v4() -> Uuid {
    Uuid::new_v4()
}

pub(crate) fn base64_encode(data: impl AsRef<[u8]>) -> String {
    BASE64_STANDARD.encode(data)
}

pub(crate) fn base64_decode(data: impl AsRef<[u8]>) -> Result<Vec<u8>, DecodeError> {
    BASE64_STANDARD.decode(data)
}
