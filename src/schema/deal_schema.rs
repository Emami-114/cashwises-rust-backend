use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDealSchema {
    pub title: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_free: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_url: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct UpdateDealSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<Vec<String>>,
    pub published: Option<bool>,
    pub is_free: Option<bool>,
    pub price: Option<f64>,
    pub offer_price: Option<f64>,
    pub expiration_date: Option<String>,
    pub provider: Option<String>,
    pub provider_url: Option<String>,
    pub thumbnail: Option<String>,
    pub images: Option<Vec<String>>,
    pub user_id: Option<String>,
    pub video_url: Option<String>,
}
