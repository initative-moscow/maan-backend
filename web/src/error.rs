use actix_web::ResponseError;

#[derive(Debug, thiserror::Error)]
pub enum AnyhowResponseError {
    #[error("{0}")]
    AnyhowError(#[from] anyhow::Error),
}

impl AnyhowResponseError {
    // pub fn inner(self) -> anyhow::Error {
    //     match self {
    //         Self::AnyhowError(err) => err,
    //     }
    // }
}

// TODO: add proper impl
impl ResponseError for AnyhowResponseError { }