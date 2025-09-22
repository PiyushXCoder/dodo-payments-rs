use super::*;
use crate::{client::Handle, operations::archive_meter::ArchiveMeterResponse};

use std::sync::Arc;

pub struct ArchiveMeterBuilder {
    pub meter_id: String,
    pub(crate) handle: Arc<Handle>,
}

impl ArchiveMeterBuilder {
    pub fn new(handle: Arc<Handle>, meter_id: String) -> Self {
        Self {
            handle,
            meter_id,
        }
    }

    pub async fn send(self) -> Result<ArchiveMeterResponse, crate::errors::Error> {
        Ok(ArchiveMeter::orchestrate(self.handle, self.meter_id).await?)
    }
}