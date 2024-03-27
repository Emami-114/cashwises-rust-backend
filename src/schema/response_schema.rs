
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}