use super::*;
use crate::{client::Handle, operations::unarchive_meter::UnarchiveMeterResponse};

use std::sync::Arc;

pub struct UnarchiveMeterBuilder {
    pub meter_id: String,
    pub(crate) handle: Arc<Handle>,
}

impl UnarchiveMeterBuilder {
    pub fn new(handle: Arc<Handle>, meter_id: String) -> Self {
        Self {
            handle,
            meter_id,
        }
    }

    pub async fn send(self) -> Result<UnarchiveMeterResponse, crate::errors::Error> {
        Ok(UnarchiveMeter::orchestrate(self.handle, self.meter_id).await?)
    }
}