use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteDealRequest {
    pub deal_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteDealResponse {
    pub deal_id: String,
}
