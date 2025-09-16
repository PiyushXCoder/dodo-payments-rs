use super::super::common::structs::BillingAddress;
use super::super::common::structs::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde_with::skip_serializing_none]
pub struct PaymentResponse {
    pub payment_id: String,
    pub business_id: String,
    pub total_amount: i32,
    pub currency: Currency,
    pub created_at: String, // date-time
    pub disputes: Vec<DisputeResponse>,
    pub refunds: Vec<RefundListItem>,
    pub customer: CustomerLimitedDetailsResponse,
    pub metadata: HashMap<String, String>,
    pub settlement_amount: i32,
    pub settlement_currency: Currency,
    pub billing: BillingAddress,
    pub brand_id: String,
    pub digital_products_delivered: bool,

    pub card_issuing_country: Option<CountryCodeAlpha2>,
    pub card_last_four: Option<String>,
    pub card_network: Option<String>,
    pub card_type: Option<String>,
    pub checkout_session_id: Option<String>,
    pub discount_id: Option<String>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub payment_link: Option<String>,
    pub payment_method: Option<String>,
    pub payment_method_type: Option<String>,
    pub product_cart: Option<Vec<OneTimeProductCartItemResponse>>,
    pub settlement_tax: Option<i32>,
    pub status: Option<IntentStatus>,
    pub subscription_id: Option<String>,
    pub tax: Option<i32>,
    pub updated_at: Option<String>, // date-time
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde_with::skip_serializing_none]
pub struct DisputeResponse {
    pub amount: String,
    pub business_id: String,
    pub created_at: String, // date-time
    pub currency: String,   // Should be Currency enum, but OpenAPI says string
    pub dispute_id: String,
    pub dispute_stage: DisputeStage,
    pub dispute_status: DisputeStatus,
    pub payment_id: String,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OneTimeProductCartItemResponse {
    pub product_id: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde_with::skip_serializing_none]
pub struct RefundListItem {
    pub refund_id: String,
    pub payment_id: String,
    pub business_id: String,
    pub status: RefundStatus,
    pub created_at: String, // date-time
    pub is_partial: bool,

    pub amount: Option<i32>,
    pub currency: Option<Currency>,
    pub reason: Option<String>,
}
