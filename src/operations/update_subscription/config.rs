use super::super::common::structs::{BillingAddress, Metadata, SubscriptionStatus};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateSubscriptionConfig {
    pub subscription_id: String,
    pub billing: Option<BillingAddress>,
    pub cancel_at_next_billing_date: Option<bool>,
    pub disable_on_demand: Option<DisableOnDemandReq>,
    pub metadata: Option<Metadata>,
    pub next_billing_date: Option<String>, // TODO: Use chrono for date-time
    pub status: Option<SubscriptionStatus>,
    pub tax_id: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DisableOnDemandReq {
    pub next_billing_date: String, // TODO: Use chrono for date-time
}
