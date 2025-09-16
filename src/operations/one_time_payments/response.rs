use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerLimitedDetailsResponse {
    pub customer_id: String,
    pub email: String,
    pub name: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateOneTimePaymentResponse {
    pub client_secret: String,
    pub customer: CustomerLimitedDetailsResponse,
    pub discount_id: Option<String>,
    pub expires_on: Option<String>,
    pub metadata: HashMap<String, String>,
    pub payment_id: String,
    pub payment_link: Option<String>,
    pub product_cart: Option<Vec<super::OneTimeProductCartItemReq>>,
    pub total_amount: i32,
}
