//! Signer module.
//!
//! Provides signers which implement [`RequestSigner`] trait.

mod dummy;
mod rsa;

pub use dummy::*;
pub use rsa::*;
