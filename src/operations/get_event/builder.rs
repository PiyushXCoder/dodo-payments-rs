use super::*;
use crate::{client::Handle, operations::get_event::GetEventResponse};

use std::sync::Arc;

pub struct GetEventBuilder {
    pub event_id: String,
    pub(crate) handle: Arc<Handle>,
}

impl GetEventBuilder {
    pub fn new(handle: Arc<Handle>, event_id: String) -> Self {
        Self {
            handle,
            event_id,
        }
    }

    pub async fn send(self) -> Result<GetEventResponse, crate::errors::Error> {
        Ok(GetEvent::orchestrate(self.handle, self.event_id).await?)
    }
}