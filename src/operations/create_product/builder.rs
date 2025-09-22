use super::*;
use crate::{client::Handle, operations::create_product::GetProductResponse};
use std::sync::Arc;
use super::super::common::structs::{CreateDigitalProductDeliveryRequest, LicenseKeyDuration, Metadata, Price, TaxCategory};

pub struct CreateProductBuilder {
    pub config: CreateProductRequest,
    pub(crate) handle: Arc<Handle>,
}

impl CreateProductBuilder {
    pub fn new(handle: Arc<Handle>, tax_category: TaxCategory, price: Price) -> Self {
        Self {
            handle,
            config: CreateProductRequest {
                tax_category,
                price,
                addons: None,
                brand_id: None,
                description: None,
                digital_product_delivery: None,
                license_key_activation_message: None,
                license_key_activations_limit: None,
                license_key_duration: None,
                license_key_enabled: None,
                metadata: None,
                name: None,
            },
        }
    }

    pub fn addons(mut self, addons: Vec<String>) -> Self {
        self.config.addons = Some(addons);
        self
    }

    pub fn brand_id(mut self, brand_id: String) -> Self {
        self.config.brand_id = Some(brand_id);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.config.description = Some(description);
        self
    }

    pub fn digital_product_delivery(mut self, digital_product_delivery: CreateDigitalProductDeliveryRequest) -> Self {
        self.config.digital_product_delivery = Some(digital_product_delivery);
        self
    }

    pub fn license_key_activation_message(mut self, license_key_activation_message: String) -> Self {
        self.config.license_key_activation_message = Some(license_key_activation_message);
        self
    }

    pub fn license_key_activations_limit(mut self, license_key_activations_limit: i32) -> Self {
        self.config.license_key_activations_limit = Some(license_key_activations_limit);
        self
    }

    pub fn license_key_duration(mut self, license_key_duration: LicenseKeyDuration) -> Self {
        self.config.license_key_duration = Some(license_key_duration);
        self
    }

    pub fn license_key_enabled(mut self, license_key_enabled: bool) -> Self {
        self.config.license_key_enabled = Some(license_key_enabled);
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.config.metadata = Some(metadata);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.config.name = Some(name);
        self
    }

    pub async fn send(self) -> Result<GetProductResponse, crate::errors::Error> {
        Ok(CreateProduct::orchestrate(self.handle, self.config).await?)
    }
}
