use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize,Deserialize,FromRow,Debug)]
pub struct TagModel {
    pub id: Uuid,
    pub title: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct CreateTagSchema {
    pub title: String,
}