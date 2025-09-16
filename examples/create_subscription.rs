use dodo_payments::{
    DodoPayments,
    client::DodoPaymentsConfigBuilder,
    operations::common::structs::*,
    operations::create_subscription::{AttachExistingCustomer, CustomerRequest, NewCustomer},
};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");
    let product_id = env::var("DODO_PAYMENTS_PRODUCT_ID")
        .expect("DODO_PAYMENTS_PRODUCT_ID must be set in env variables");
    let customer_id = env::var("DODO_PAYMENTS_CUSTOMER_ID").ok(); // Optional customer ID

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let customer = if let Some(id) = customer_id {
        CustomerRequest::AttachExisting(AttachExistingCustomer { customer_id: id })
    } else {
        CustomerRequest::New(NewCustomer {
            email: "test@example.com".to_string(),
            name: "Test Customer".to_string(),
            phone_number: None,
        })
    };

    let billing_address = BillingAddress {
        city: "Test City".to_string(),
        country: CountryCodeAlpha2::US,
        state: "Test State".to_string(),
        street: "123 Test St".to_string(),
        zipcode: "12345".to_string(),
    };

    let response = client
        .create_subscription(
            product_id,
            1, // quantity
            customer,
            billing_address,
        )
        .send()
        .await;

    println!("Response: {:#?}", response);
}
