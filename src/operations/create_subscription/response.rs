use super::super::common::structs::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddonCartResponseItem {
    pub addon_id: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseMetadata(pub HashMap<String, String>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateSubscriptionResponse {
    pub subscription_id: String,
    pub recurring_pre_tax_amount: i32,
    pub customer: CustomerLimitedDetailsResponse,
    pub metadata: ResponseMetadata,
    pub addons: Vec<AddonCartResponseItem>,
    pub payment_id: String,
    pub client_secret: Option<String>,
    pub discount_id: Option<String>,
    pub expires_on: Option<String>,
    pub payment_link: Option<String>,
}
