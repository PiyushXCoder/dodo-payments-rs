use super::super::common::structs::{MeterAggregation, MeterFilter};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct CreateMeterConfig {
    #[validate(min_length = 1)]
    pub name: String,
    
    #[validate(min_length = 1)]
    pub event_name: String,
    
    pub aggregation: MeterAggregation,
    
    #[validate(min_length = 1)]
    pub measurement_unit: String,
    
    pub description: Option<String>,
    
    pub filter: Option<MeterFilter>,
}