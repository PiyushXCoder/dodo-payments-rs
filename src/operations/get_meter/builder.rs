use super::*;
use crate::{client::Handle, operations::get_meter::GetMeterResponse};

use std::sync::Arc;

pub struct GetMeterBuilder {
    pub meter_id: String,
    pub(crate) handle: Arc<Handle>,
}

impl GetMeterBuilder {
    pub fn new(handle: Arc<Handle>, meter_id: String) -> Self {
        Self {
            handle,
            meter_id,
        }
    }

    pub async fn send(self) -> Result<GetMeterResponse, crate::errors::Error> {
        Ok(GetMeter::orchestrate(self.handle, self.meter_id).await?)
    }
}