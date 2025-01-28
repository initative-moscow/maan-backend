use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ListDealsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    pub field_names: Vec<String>,
    pub filters: ListDealsRequestFilters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListDealsRequestFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListDealsReponse {
    pub deals: Vec<ListDealsResponseDeal>,
    // TODO
    pub meta: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListDealsResponseDeal {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_key: Option<String>,
}
