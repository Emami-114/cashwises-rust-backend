use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct CategoryModel {
    pub id: Uuid,
    pub title: String,
    pub thumbnail: Option<String>,
    pub user_id: Option<String>,
    pub published: Option<bool>,
    pub status: Option<String>,
    pub main_id: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
