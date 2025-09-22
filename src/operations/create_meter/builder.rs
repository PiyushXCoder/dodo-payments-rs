use super::super::common::structs::{MeterAggregation, MeterFilter};
use super::*;
use crate::{client::Handle, operations::create_meter::CreateMeterResponse};

use std::sync::Arc;

pub struct CreateMeterBuilder {
    pub config: CreateMeterConfig,
    pub(crate) handle: Arc<Handle>,
}

impl CreateMeterBuilder {
    pub fn new(
        handle: Arc<Handle>,
        name: String,
        event_name: String,
        aggregation: MeterAggregation,
        measurement_unit: String,
    ) -> Self {
        Self {
            handle,
            config: CreateMeterConfig {
                name,
                event_name,
                aggregation,
                measurement_unit,
                description: None,
                filter: None,
            },
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.config.name = name;
        self
    }

    pub fn event_name(mut self, event_name: String) -> Self {
        self.config.event_name = event_name;
        self
    }

    pub fn aggregation(mut self, aggregation: MeterAggregation) -> Self {
        self.config.aggregation = aggregation;
        self
    }

    pub fn measurement_unit(mut self, measurement_unit: String) -> Self {
        self.config.measurement_unit = measurement_unit;
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.config.description = Some(description);
        self
    }

    pub fn filter(mut self, filter: MeterFilter) -> Self {
        self.config.filter = Some(filter);
        self
    }

    pub async fn send(self) -> Result<CreateMeterResponse, crate::errors::Error> {
        Ok(CreateMeter::orchestrate(self.handle, self.config).await?)
    }
}