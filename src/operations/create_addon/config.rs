use super::super::common::structs::{Currency, TaxCategory};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct CreateAddonConfig {
    #[validate(min_length = 1)]
    pub name: String,
    
    pub tax_category: TaxCategory,
    
    #[validate(minimum = 0)]
    pub price: i32,
    
    pub currency: Currency,
    
    pub description: Option<String>,
}