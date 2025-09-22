use super::*;
use crate::{client::Handle, operations::create_product::GetProductResponse};
use std::sync::Arc;
use super::super::common::structs::{LicenseKeyDuration, Metadata, PatchDigitalProductDeliveryRequest, Price, TaxCategory};

pub struct UpdateProductBuilder {
    pub config: UpdateProductConfig,
    pub(crate) handle: Arc<Handle>,
}

impl UpdateProductBuilder {
    pub fn new(handle: Arc<Handle>, id: String) -> Self {
        Self {
            handle,
            config: UpdateProductConfig {
                id,
                body: PatchProductRequest {
                    addons: None,
                    brand_id: None,
                    description: None,
                    digital_product_delivery: None,
                    image_id: None,
                    license_key_activation_message: None,
                    license_key_activations_limit: None,
                    license_key_duration: None,
                    license_key_enabled: None,
                    metadata: None,
                    name: None,
                    price: None,
                    tax_category: None,
                },
            },
        }
    }

    pub fn addons(mut self, addons: Vec<String>) -> Self {
        self.config.body.addons = Some(addons);
        self
    }

    pub fn brand_id(mut self, brand_id: String) -> Self {
        self.config.body.brand_id = Some(brand_id);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.config.body.description = Some(description);
        self
    }

    pub fn digital_product_delivery(mut self, digital_product_delivery: PatchDigitalProductDeliveryRequest) -> Self {
        self.config.body.digital_product_delivery = Some(digital_product_delivery);
        self
    }

    pub fn image_id(mut self, image_id: String) -> Self {
        self.config.body.image_id = Some(image_id);
        self
    }

    pub fn license_key_activation_message(mut self, license_key_activation_message: String) -> Self {
        self.config.body.license_key_activation_message = Some(license_key_activation_message);
        self
    }

    pub fn license_key_activations_limit(mut self, license_key_activations_limit: i32) -> Self {
        self.config.body.license_key_activations_limit = Some(license_key_activations_limit);
        self
    }

    pub fn license_key_duration(mut self, license_key_duration: LicenseKeyDuration) -> Self {
        self.config.body.license_key_duration = Some(license_key_duration);
        self
    }

    pub fn license_key_enabled(mut self, license_key_enabled: bool) -> Self {
        self.config.body.license_key_enabled = Some(license_key_enabled);
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.config.body.metadata = Some(metadata);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.config.body.name = Some(name);
        self
    }

    pub fn price(mut self, price: Price) -> Self {
        self.config.body.price = Some(price);
        self
    }

    pub fn tax_category(mut self, tax_category: TaxCategory) -> Self {
        self.config.body.tax_category = Some(tax_category);
        self
    }

    pub async fn send(self) -> Result<GetProductResponse, crate::errors::Error> {
        Ok(UpdateProduct::orchestrate(self.handle, self.config).await?)
    }
}
