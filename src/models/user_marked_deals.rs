use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserMarkedDeals {
    pub user_id: Uuid,
    pub deal_id: Uuid,
}