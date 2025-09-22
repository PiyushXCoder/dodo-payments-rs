use serde::{Deserialize, Serialize};
use super::super::common::structs::{CreateDigitalProductDeliveryRequest, LicenseKeyDuration, Metadata, Price, TaxCategory};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateProductRequest {
    pub tax_category: TaxCategory,
    pub price: Price,
    pub addons: Option<Vec<String>>,
    pub brand_id: Option<String>,
    pub description: Option<String>,
    pub digital_product_delivery: Option<CreateDigitalProductDeliveryRequest>,
    pub license_key_activation_message: Option<String>,
    pub license_key_activations_limit: Option<i32>,
    pub license_key_duration: Option<LicenseKeyDuration>,
    pub license_key_enabled: Option<bool>,
    pub metadata: Option<Metadata>,
    pub name: Option<String>,
}
