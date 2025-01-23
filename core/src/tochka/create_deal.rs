use serde::{Deserialize, Serialize};
use super::identification_payment::PaymentOwner;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDealRequest {
    pub ext_key: String,
    pub amount: f64,
    pub payers: Vec<PaymentOwner>,
    pub recipients: Vec<DealRecipient>, 
}

pub enum DealRecipient {
    PaymentContract {

    }
}

