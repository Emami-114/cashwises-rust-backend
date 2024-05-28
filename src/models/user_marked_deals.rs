use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserMarkedDeals {
    pub user_id: Uuid,
    pub deal_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UserMarkedDealsSchema {
    pub user_id: String,
    pub deal_id: String,
}