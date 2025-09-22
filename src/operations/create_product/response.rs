use serde::{Deserialize, Serialize};
use super::super::common::structs::{DigitalProductDelivery, LicenseKeyDuration, Metadata, Price, TaxCategory};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetProductResponse {
    pub product_id: String,
    pub business_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_recurring: bool,
    pub tax_category: TaxCategory,
    pub price: Price,
    pub license_key_enabled: bool,
    pub brand_id: Option<String>,
    pub metadata: Metadata,
    pub addons: Option<Vec<String>>,
    pub description: Option<String>,
    pub digital_product_delivery: Option<DigitalProductDelivery>,
    pub image: Option<String>,
    pub license_key_activation_message: Option<String>,
    pub license_key_activations_limit: Option<i32>,
    pub license_key_duration: Option<LicenseKeyDuration>,
    pub name: Option<String>,
}
