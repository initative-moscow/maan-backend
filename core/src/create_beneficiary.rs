//! Create beneficiary request.

use crate::{RequestSigner, TochkaApiResponse, TochkaCyclopsClient};
use anyhow::Result as AnyhowResult;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

impl<S> TochkaCyclopsClient<S> {
    /// Create a new beneficiary.
    pub async fn create_beneficiary_ul(
        &self,
        params: CreateBeneficiaryUlRequest,
    ) -> AnyhowResult<TochkaApiResponse<CreateBeneficiaryResponse>>
    where
        S: RequestSigner<Vec<u8>>,
        <S as RequestSigner<Vec<u8>>>::Error: StdError + Send + Sync + 'static,
    {
        self.send_request::<_, CreateBeneficiaryResponse>("create_beneficiary_ul", params)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CreateBeneficiaryResponse {
    #[serde(rename = "beneficiary")]
    Beneficiary {
        inn: String,
        nominal_account_code: String,
        nominal_account_bic: String,
        id: String,
    },
}

/// Create beneficiary ul request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateBeneficiaryUlRequest {
    pub inn: String,
    pub nominal_account_code: String,
    pub nominal_account_bic: String,
    pub beneficiary_data: BeneficiaryData,
}

/// Beneficiary data for creating beneficiary request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BeneficiaryData {
    pub name: String,
    pub kpp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ogrn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_branch: Option<bool>,
}

#[cfg(test)]
mod tests {
    // TODO IO tests required
}
