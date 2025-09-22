use super::super::common::structs::{Currency, TaxCategory};
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateAddonConfig {
    pub name: Option<String>,
    pub tax_category: Option<TaxCategory>,
    pub price: Option<i32>,
    pub currency: Option<Currency>,
    pub description: Option<String>,
    pub image_id: Option<String>,
}