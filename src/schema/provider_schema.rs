use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProvider {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Deserialize,Debug)]
pub struct ProviderFilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub query: Option<String>,
}