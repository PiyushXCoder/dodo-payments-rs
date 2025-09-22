use super::super::common::structs::{MeterAggregation, MeterFilter};
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateMeterResponse {
    pub id: String,
    pub business_id: String,
    pub name: String,
    pub event_name: String,
    pub aggregation: MeterAggregation,
    pub measurement_unit: String,
    pub description: Option<String>,
    pub filter: Option<MeterFilter>,
    pub created_at: String,
    pub updated_at: String,
}