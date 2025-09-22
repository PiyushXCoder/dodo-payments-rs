use serde::{Deserialize, Serialize};
use super::super::common::structs::{LicenseKeyDuration, Metadata, PatchDigitalProductDeliveryRequest, Price, TaxCategory};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatchProductRequest {
    pub addons: Option<Vec<String>>,
    pub brand_id: Option<String>,
    pub description: Option<String>,
    pub digital_product_delivery: Option<PatchDigitalProductDeliveryRequest>,
    pub image_id: Option<String>,
    pub license_key_activation_message: Option<String>,
    pub license_key_activations_limit: Option<i32>,
    pub license_key_duration: Option<LicenseKeyDuration>,
    pub license_key_enabled: Option<bool>,
    pub metadata: Option<Metadata>,
    pub name: Option<String>,
    pub price: Option<Price>,
    pub tax_category: Option<TaxCategory>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateProductConfig {
    pub id: String,
    #[serde(flatten)]
    pub body: PatchProductRequest,
}
