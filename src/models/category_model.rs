use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;


#[derive(Debug,FromRow,Deserialize,Serialize)]
#[allow(non_snake_case)]
pub struct CategoryModel {
    pub id: Uuid,
    pub title: String,
    pub thumbnail: String,
    pub user_id: Option<String>,
    pub published: Option<bool>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>
}

