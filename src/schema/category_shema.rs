use serde::{Deserialize, Serialize};



#[derive(Serialize,Deserialize,Debug)]
pub struct CreateCategorySchema {
    pub title: String,
    pub thumbnail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub published: Option<bool>
}