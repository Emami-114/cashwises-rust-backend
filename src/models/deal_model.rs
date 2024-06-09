use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize)]
#[allow(non_snake_case)]
pub struct DealModel {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub categories: Option<Vec<String>>,
    pub is_free: Option<bool>,
    pub price: Option<f64>,
    pub offer_price: Option<f64>,
    pub published: Option<bool>,
    pub expiration_date: Option<String>,
    pub provider: Option<String>,
    pub provider_url: Option<String>,
    pub thumbnail: Option<String>,
    pub images: Option<Vec<String>>,
    pub user_id: Option<String>,
    pub tags: Option<Vec<String>>,
    pub shipping_costs: Option<f64>,
    pub video_url: Option<String>,
    pub coupon_code: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Serialize for DealModel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("DealModel", 20)?;
        let _id = state.serialize_field("id", &self.id)?;

        let _title = state.serialize_field("title", &self.title)?;

        let _description = state.serialize_field("description", &self.description)?;

        if let Some(categories) = self.categories.to_owned() {
            state.serialize_field("categories", &categories)?;
        }
        if let Some(is_free) = self.is_free {
            state.serialize_field("is_free", &is_free)?;
        }
        if let Some(price) = self.price {
            state.serialize_field("price", &price)?;
        }
        if let Some(offer_price) = self.offer_price {
            state.serialize_field("offer_price", &offer_price)?;
        }
        if let Some(published) = self.published {
            state.serialize_field("published", &published)?;
        }
        if let Some(thumbnail) = self.thumbnail.to_owned() {
            state.serialize_field("thumbnail", &thumbnail)?;
        }
        if let Some(images) = self.images.to_owned() {
            state.serialize_field("images", &images)?;
        }
        if let Some(expiration_date) = self.expiration_date.to_owned() {
            state.serialize_field("expiration_date", &expiration_date)?;
        }
        if let Some(provider) = self.provider.to_owned() {
            state.serialize_field("provider", &provider)?;
        }
        if let Some(provider_url) = self.provider_url.to_owned() {
            state.serialize_field("provider_url", &provider_url)?;
        }
        if let Some(user_id) = self.user_id.to_owned() {
            state.serialize_field("user_id", &user_id)?;
        }
        if let Some(video_url) = self.video_url.to_owned() {
            state.serialize_field("video_url", &video_url)?;
        }
        if let Some(tags) = self.tags.to_owned() {
            state.serialize_field("tags", &tags)?;
        }
        if let Some(shipping_costs) = self.shipping_costs.to_owned() {
            state.serialize_field("shipping_costs", &shipping_costs)?;
        }
        if let Some(coupon_code) = self.coupon_code.to_owned() {
            state.serialize_field("coupon_code", &coupon_code)?;
        }
        state.serialize_field("created_at", &self.created_at)?;
        state.serialize_field("updated_at", &self.updated_at)?;
        state.end()
    }
}

#[derive(Debug, FromRow, Deserialize,Serialize)]
#[allow(non_snake_case)]
pub struct SmallDealModel {
    pub id: Uuid,
    pub title: String,
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
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_costs: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
