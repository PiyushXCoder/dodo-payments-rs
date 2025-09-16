use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct AttachAddonReq {
    pub addon_id: String,
    #[validate(minimum = 0)]
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ProrationBillingMode {
    ProratedImmediately,
    FullImmediately,
    DifferenceImmediately,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct ChangePlanConfig {
    pub product_id: String,
    pub proration_billing_mode: ProrationBillingMode,
    #[validate(minimum = 0)]
    pub quantity: i32,
    pub addons: Option<Vec<AttachAddonReq>>,
}
