use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder, operations::common::structs::{Price, TaxCategory, OneTimePrice, Currency}};
use std::env;

#[tokio::main]
async fn main() {
    let bearer_token = env::var("DODO_PAYMENTS_BEARER_TOKEN")
        .expect("DODO_PAYMENTS_BEARER_TOKEN must be set in env variables");

    let client = DodoPayments::new(
        DodoPaymentsConfigBuilder::new()
            .bearer_token(&bearer_token)
            .environment("test_mode"),
    );

    let price = Price::OneTime {
        type_field: "one_time_price".to_string(),
        one_time_price: OneTimePrice {
            price: 1000,
            currency: Currency::Usd,
            discount: 0,
            purchasing_power_parity: false,
            pay_what_you_want: Some(false),
            suggested_price: None,
            tax_inclusive: Some(false),
        },
    };

    let response = client
        .create_product(TaxCategory::DigitalProducts, price)
        .name("Example Product".to_string())
        .description("This is an example product.".to_string())
        .send()
        .await;

    println!("Response: {:#?}", response);
}
