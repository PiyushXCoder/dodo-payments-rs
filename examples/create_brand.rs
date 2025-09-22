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
        .create_brand()
        .name("My Awesome Brand".to_string())
        .description("This is an awesome brand for selling digital products".to_string())
        .statement_descriptor("Awesome Brand Store".to_string())
        .support_email("support@awesomebrand.com".to_string())
        .url("https://www.awesomebrand.com".to_string())
        .send()
        .await;

    match response {
        Ok(brand) => {
            println!("Brand created successfully!");
            println!("Brand ID: {}", brand.brand_id);
            println!("Business ID: {}", brand.business_id);
            println!("Name: {:?}", brand.name);
            println!("Statement Descriptor: {}", brand.statement_descriptor);
            println!("Verification Status: {:?}", brand.verification_status);
            println!("Enabled: {}", brand.enabled);
            
            if let Some(description) = brand.description {
                println!("Description: {}", description);
            }
            
            if let Some(support_email) = brand.support_email {
                println!("Support Email: {}", support_email);
            }
            
            if let Some(url) = brand.url {
                println!("URL: {}", url);
            }
        },
        Err(e) => println!("Failed to create brand: {:?}", e),
    }

    // Create a minimal brand with only required fields
    let response2 = client
        .create_brand()
        .statement_descriptor("Minimal Brand".to_string())
        .send()
        .await;

    match response2 {
        Ok(brand) => {
            println!("\nMinimal brand created successfully!");
            println!("Brand ID: {}", brand.brand_id);
            println!("Statement Descriptor: {}", brand.statement_descriptor);
            println!("Verification Status: {:?}", brand.verification_status);
        },
        Err(e) => println!("Failed to create minimal brand: {:?}", e),
    }
}