use std::fmt;
use std::fmt::Write;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde::de::Visitor;

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub query: Option<String>,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
}

impl<'de> Deserialize<'de> for FilterOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TempFilterOptions {
            page: Option<usize>,
            limit: Option<usize>,
            query: Option<String>,
            tags: Option<String>,
            categories: Option<String>,
        }

        let temp = TempFilterOptions::deserialize(deserializer)?;
        let tags = temp.tags.map(|t| t.split(',').map(String::from).collect());
        let categories = temp.categories.map(|t| t.split(',').map(String::from).collect());

        Ok(FilterOptions {
            page: temp.page,
            limit: temp.limit,
            query: temp.query,
            tags,
            categories,
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct ImageOptions {
    #[serde(rename = "dir")]
    pub dir: String,
}
