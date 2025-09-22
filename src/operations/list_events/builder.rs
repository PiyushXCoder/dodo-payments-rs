use super::*;
use crate::{client::Handle, operations::list_events::ListEventsResponse};

use std::sync::Arc;

pub struct ListEventsBuilder {
    pub config: ListEventsConfig,
    pub(crate) handle: Arc<Handle>,
}

impl ListEventsBuilder {
    pub fn new(handle: Arc<Handle>) -> Self {
        Self {
            handle,
            config: ListEventsConfig {
                customer_id: None,
                event_name: None,
                meter_id: None,
                page_size: None,
                page_number: None,
                start: None,
                end: None,
            },
        }
    }

    pub fn customer_id(mut self, customer_id: String) -> Self {
        self.config.customer_id = Some(customer_id);
        self
    }

    pub fn event_name(mut self, event_name: String) -> Self {
        self.config.event_name = Some(event_name);
        self
    }

    pub fn meter_id(mut self, meter_id: String) -> Self {
        self.config.meter_id = Some(meter_id);
        self
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.config.page_size = Some(page_size);
        self
    }

    pub fn page_number(mut self, page_number: u32) -> Self {
        self.config.page_number = Some(page_number);
        self
    }

    pub fn start(mut self, start: String) -> Self {
        self.config.start = Some(start);
        self
    }

    pub fn end(mut self, end: String) -> Self {
        self.config.end = Some(end);
        self
    }

    pub async fn send(self) -> Result<ListEventsResponse, crate::errors::Error> {
        Ok(ListEvents::orchestrate(self.handle, self.config).await?)
    }
}