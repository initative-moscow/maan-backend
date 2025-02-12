//! Create beneficiary request.

use crate::{RequestSigner, TochkaApiResponse, TochkaCyclopsClient};
use anyhow::Result as AnyhowResult;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

impl<S> TochkaCyclopsClient<S> {
    /// Create a new beneficiary of UL type.
    pub async fn create_beneficiary_ul(
        &self,
        params: CreateBeneficiaryUlRequest,
    ) -> AnyhowResult<TochkaApiResponse<CreateBeneficiaryUlResponse>>
    where
        S: RequestSigner<Vec<u8>>,
        <S as RequestSigner<Vec<u8>>>::Error: StdError + Send + Sync + 'static,
    {
        self.send_request::<_, CreateBeneficiaryUlResponse>("create_beneficiary_ul", params)
            .await
    }
}

/// A response from `create_beneficiary_ul` method.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateBeneficiaryUlResponse {
    pub beneficiary: CreatedUlBeneficiary,
}

/// A beneficiary received from the `create_beneficiary_ul` method.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreatedUlBeneficiary {
    /// Beneficiary's INN number.
    pub inn: String,
    /// Nominal account code under which the beneficiary is registered.
    pub nominal_account_code: String,
    /// Nominal account bic under which the beneficiary is registered.
    pub nominal_account_bic: String,
    /// Beneficiary's ID.
    pub id: String,
}

/// Create beneficiary ul request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateBeneficiaryUlRequest {
    /// Beneficiary's INN number.
    pub inn: String,
    /// Nominal account code under which the beneficiary is registered.
    pub nominal_account_code: String,
    /// Nominal account bic under which the beneficiary is registered.
    pub nominal_account_bic: String,
    /// Beneficiary's data.
    pub beneficiary_data: BeneficiaryData,
}

/// Beneficiary identification data except for INN.
///
/// Beneficiary organizations can have same name, KPP
/// and OGRN, but different INNs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BeneficiaryData {
    /// Organization name.
    pub name: String,
    /// Organization KPP.
    pub kpp: String,
    /// Organization OGRN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ogrn: Option<String>,
    /// Flag states if this beneficiary is a branch
    /// of some already known in Tochka cyclops organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_branch: Option<bool>,
}

#[cfg(test)]
mod tests {
    use crate::{tochka_io::*, *};
    use anyhow::{Context, Result};

    fn generate_random_inn_j() -> String {
        use rand::Rng;

        let base: String = (0..9)
            .map(|_| rand::thread_rng().gen_range(0..10).to_string())
            .collect();
        let coefficients = [2u32, 4, 10, 3, 5, 9, 4, 6, 8];
        let checksum = (base.as_str())
            .chars()
            .zip(coefficients.iter())
            .map(|(inn_char, coeff)| inn_char.to_digit(10).expect("invalid digit") * coeff)
            .sum::<u32>()
            % 11
            % 10;

        format!("{}{}", base, checksum)
    }

    #[tokio::test]
    async fn create_valid_beneficiary() -> Result<()> {
        let (args, client) = test_utils::new_test_client()?;

        let params = CreateBeneficiaryUlRequest {
            inn: generate_random_inn_j(),
            nominal_account_code: args.nominal_account_code,
            nominal_account_bic: args.nominal_account_bic,
            beneficiary_data: BeneficiaryData {
                name: "ООО \"Петруня\"".to_string(),
                kpp: "667101001".to_string(),
                ogrn: None,
                is_branch: None,
            },
        };
        let resp = client
            .create_beneficiary_ul(params)
            .await
            .context("failed sending request")?;
        assert!(resp.payload.is_ok());

        Ok(())
    }

    // TODO add failing tests
}
