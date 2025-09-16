use dodo_payments::{
    DodoPayments,
    client::DodoPaymentsConfigBuilder,
    operations::{
        checkout_sessions::BillingAddress,
        one_time_payments::{AttachExistingCustomer, CustomerRequest, OneTimeProductCartItemReq},
    },
};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
    let product_id = env::var("DODO_PAYMENTS_PRODUCT_ID")
        .expect("DODO_PAYMENTS_PRODUCT_ID must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let product_cart = vec![OneTimeProductCartItemReq {
        product_id: product_id.clone(),
        quantity: 1,
        amount: Some(100),
    }];

    let customer = CustomerRequest::AttachExisting(AttachExistingCustomer {
        customer_id: "cus_GT2zxxOOwZ6LxWiwWg79d".to_string(),
    });

    let billing = BillingAddress {
        street: "123 Main St".to_string(),
        city: "Anytown".to_string(),
        state: "CA".to_string(),
        country: "US".to_string(),
        zipcode: "90210".to_string(),
    };

    let response = client
        .one_time_payments(product_cart, customer, billing)
        .send()
        .await;

    println!("Response: {:?}", response);
}
