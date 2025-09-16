use dodo_payments::{
    DodoPayments, client::DodoPaymentsConfigBuilder, operations::checkout_sessions::ProductItem,
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

    let response = client
        .checkout_sessions()
        .product_cart(vec![ProductItem {
            product_id: product_id,
            quantity: 1,
        }])
        .send()
        .await;

    println!("Response: {:?}", response);
}
