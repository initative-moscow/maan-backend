//! Testing utils

use crate::{RsaSigner, TochkaCyclopsClient};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::{fs, path::PathBuf};

const CONFIG_PATH: &str = "./.api_test_config.toml";

pub(crate) fn new_test_client() -> Result<(TestArgs, TochkaCyclopsClient<RsaSigner>)> {
    let args = TestArgs::new()?;
    let signer = RsaSigner::new(args.private_key_path.clone())?;
    let client = TochkaCyclopsClient::new(
        args.sign_system.clone(),
        args.sign_thumbprint.clone(),
        "https://pre.tochka.com/api/v1/cyclops".to_string(),
        signer,
    );

    Ok((args, client))
}

#[derive(Debug, Deserialize)]
pub(crate) struct TestArgs {
    pub(crate) private_key_path: PathBuf,
    pub(crate) sign_system: String,
    pub(crate) sign_thumbprint: String,
    pub(crate) nominal_account_code: String,
    pub(crate) nominal_account_bic: String,
}

impl TestArgs {
    pub(crate) fn new() -> Result<Self> {
        let config_content =
            fs::read_to_string(PathBuf::from(CONFIG_PATH)).context("can't read config file")?;
        toml::from_str::<TestArgs>(&config_content)
            .context("failed to parse into `Params`")
            .map_err(Into::into)
    }
}
