use super::super::common::structs::EventInput;
use super::*;
use crate::{client::Handle, operations::ingest_events::IngestEventsResponse};

use std::sync::Arc;

pub struct IngestEventsBuilder {
    pub config: IngestEventsConfig,
    pub(crate) handle: Arc<Handle>,
}

impl IngestEventsBuilder {
    pub fn new(handle: Arc<Handle>, events: Vec<EventInput>) -> Self {
        Self {
            handle,
            config: IngestEventsConfig { events },
        }
    }

    pub fn events(mut self, events: Vec<EventInput>) -> Self {
        self.config.events = events;
        self
    }

    pub async fn send(self) -> Result<IngestEventsResponse, crate::errors::Error> {
        Ok(IngestEvents::orchestrate(self.handle, self.config).await?)
    }
}