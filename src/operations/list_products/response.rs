use serde::{Deserialize, Serialize};
use super::super::common::structs::{Currency, Metadata, Price, TaxCategory};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetProductsListResponseItem {
    pub business_id: String,
    pub created_at: String,
    pub currency: Option<Currency>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub is_recurring: bool,
    pub metadata: Metadata,
    pub name: Option<String>,
    pub price: Option<i32>,
    pub price_detail: Option<Price>,
    pub product_id: String,
    pub tax_category: TaxCategory,
    pub tax_inclusive: Option<bool>,
    pub updated_at: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetProductsListResponse {
    pub items: Vec<GetProductsListResponseItem>,
}
