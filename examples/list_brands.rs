use dodo_payments::{DodoPayments, client::DodoPaymentsConfigBuilder};
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

    let response = client
        .list_brands()
        .send()
        .await;

    match response {
        Ok(brands) => {
            println!("Found {} brands", brands.items.len());
            for brand in brands.items {
                println!("- Brand ID: {}, Name: {:?}", brand.brand_id, brand.name);
                println!("  Enabled: {}, Verification Status: {:?}", brand.enabled, brand.verification_status);
                println!("  Statement Descriptor: {}", brand.statement_descriptor);
                
                if let Some(description) = brand.description {
                    println!("  Description: {}", description);
                }
                
                if let Some(url) = brand.url {
                    println!("  URL: {}", url);
                }
                
                println!();
            }
        },
        Err(e) => println!("Failed to list brands: {:?}", e),
    }
}